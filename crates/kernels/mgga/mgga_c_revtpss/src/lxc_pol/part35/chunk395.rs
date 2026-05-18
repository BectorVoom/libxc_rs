//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 395/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk395<F: Float>(t30: F, t33: F, t265: F, t393: F, t502: F, t2071: F, t207: F, t2070: F, t198: F, t892: F, t1940: F, t45: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t2072 = t2071 * t30;
    let t2075 = t207 * t2070;
    let t2077 = t198 * t2075 * t892;
    let t2078 = piecewise3::<f64>(t394, F::new(0.0), t2077);
    let t2081 = piecewise3::<f64>(t120, t1940 * t2072 / F::new(2.0), t2078 * t45 / F::new(2.0));
    let t2082 = t2071 * t33;
    let t2085 = piecewise3::<f64>(t503, F::new(0.0), t2077);
    let t2088 = piecewise3::<f64>(t400, t1940 * t2082 / F::new(2.0), t2085 * t57 / F::new(2.0));
    let t2089 = t2081 + t2088;
    (t2072, t2075, t2078, t2082, t2085, t2089)
}
