//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 423/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk423<F: Float>(t30: F, t265: F, t393: F, t1995: F, t1966: F, t45: F, t343: F, t55: F, t136: F, t473: F, dens_threshold: F, rho0: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t2129 = piecewise3::<F>(t394, F::cast_from(0.0_f64), t1995);
    let t2132 = piecewise3::<F>(t120, t1966, t2129 * t45 / F::cast_from(2.0_f64));
    let t2133 = t55 * t343;
    let t2134 = t2133 * t136;
    let t2137 = t473 * sigma2;
    (t2129, t2132, t2133, t2134, t2137)
}
