//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1300/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1300<F: Float>(t10022: F, t786: F, t3923: F, t675: F, t268: F, t4003: F, t2435: F, t4093: F, t4083: F, t9303: F, t4066: F, t545: F) -> (F, F, F, F, F) {
    let t10023 = t786 * t10022;
    let t10024 = t675 * t3923;
    let t10026 = t268 * t10024 * t4003;
    let t10027 = t10023 * t10026;
    let t10032 = t2435 * t4093;
    let t10035 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t4083;
    let t10039 = t545 * t4066;
    (t10024, t10027, t10032, t10035, t10039)
}
