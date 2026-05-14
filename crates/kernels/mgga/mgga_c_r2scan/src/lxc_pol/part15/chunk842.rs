//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 842/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk842<F: Float>(t10615: F, t3275: F, t3277: F, t3270: F, t3348: F, t3269: F, t2259: F, t797: F, t3276: F, t2330: F, t6897: F, t3263: F, t1234: F, t3262: F, t3264: F, t792: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10617 = t3275 * t10615 * t3277;
    let t10618 = 5.0 / 8.0 * t10617;
    let t10619 = t3270 * t3348;
    let t10620 = t3269 * t10619;
    let t10621 = t10620 / 2.0;
    let t10622 = t797 * t2259;
    let t10624 = t3275 * t3276 * t10622;
    let t10625 = 5.0 / 16.0 * t10624;
    let t10626 = t6897 * t2330;
    let t10628 = t3275 * t3263 * t10626;
    let t10629 = t10628 / 2.0;
    let t10630 = t797 * t1234;
    let t10632 = t3262 * t3263 * t10630;
    let t10633 = 3.0 / 4.0 * t10632;
    let t10634 = t3264 * t792;
    (t10618, t10619, t10621, t10622, t10625, t10626, t10629, t10630, t10633, t10634)
}
