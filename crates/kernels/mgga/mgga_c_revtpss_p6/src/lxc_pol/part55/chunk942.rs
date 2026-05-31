//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 942/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk942<F: Float>(t28271: F, t572: F, t1459: F, t7953: F, t116: F, t7741: F, t670: F, t117: F, t28042: F, t2106: F, t9593: F) -> (F, F, F, F, F, F, F) {
    let t28273 = F::cast_from(6.0_f64) * t572 * t28271;
    let t28275 = F::cast_from(3.0_f64) * t1459 * t7953;
    let t28276 = t116 * t7741;
    let t28277 = t28276 * t670;
    let t28279 = F::cast_from(6.0_f64) * t572 * t28277;
    let t28280 = t117 * t28042;
    let t28282 = F::cast_from(3.0_f64) * t572 * t28280;
    let t28286 = t2106 * t9593;
    (t28273, t28275, t28277, t28279, t28280, t28282, t28286)
}
