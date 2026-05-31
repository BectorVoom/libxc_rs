//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 916/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk916<F: Float>(t10615: F, t3275: F, t3277: F, t3270: F, t3348: F, t3269: F, t2259: F, t797: F, t3276: F, t2330: F, t6897: F, t3263: F) -> (F, F, F, F, F, F, F) {
    let t10617 = t3275 * t10615 * t3277;
    let t10618 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t10617;
    let t10619 = t3270 * t3348;
    let t10620 = t3269 * t10619;
    let t10621 = t10620 / F::cast_from(2.0_f64);
    let t10622 = t797 * t2259;
    let t10624 = t3275 * t3276 * t10622;
    let t10625 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t10624;
    let t10626 = t6897 * t2330;
    let t10628 = t3275 * t3263 * t10626;
    (t10618, t10619, t10621, t10622, t10625, t10626, t10628)
}
