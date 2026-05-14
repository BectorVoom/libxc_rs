//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 777/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk777<F: Float>(t30: F, t265: F, t393: F, t1518: F, t26123: F, t572: F, t4292: F, t7330: F, t1459: F, t7953: F, t116: F, t7741: F, t670: F, t117: F, t28042: F, t27754: F, t1469: F, t2129: F, t27408: F, t4186: F, t45: F, t606: F, t7594: F, t8161: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t28268 = t26123 * t1518;
    let t28270 = 6.0 * t572 * t28268;
    let t28271 = t7330 * t4292;
    let t28273 = 6.0 * t572 * t28271;
    let t28275 = 3.0 * t1459 * t7953;
    let t28276 = t116 * t7741;
    let t28277 = t28276 * t670;
    let t28279 = 6.0 * t572 * t28277;
    let t28280 = t117 * t28042;
    let t28282 = 3.0 * t572 * t28280;
    let t28998 = piecewise3(t394, 0.0, t27754);
    let t29005 = piecewise3(t120, t27408, t7594 * t1469 / 2.0 + t2129 * t4186 / 2.0 + t28998 * t45 / 2.0 + t8161 * t606 / 2.0);
    (t28268, t28270, t28271, t28273, t28275, t28277, t28279, t28280, t28282, t29005)
}
