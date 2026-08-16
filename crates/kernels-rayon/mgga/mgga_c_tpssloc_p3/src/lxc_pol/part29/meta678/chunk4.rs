//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2274/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2274(t25: f64, t265: f64, t394: f64, t89823: f64, t12606: f64, t1409: f64, t2116: f64, t2250: f64, t24555: f64, t27373: f64, t3966: f64, t40: f64, t607: f64, t7274: f64, t7992: f64, t88003: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t94281 = piecewise3(t395, 0.0_f64, t89823);
    let t94293 = piecewise3(t115, t88003, t94281 * t40 / 2.0_f64 + t27373 * t607 + t7992 * t2250 / 2.0_f64 + t24555 * t1409 / 2.0_f64 + t7274 * t3966 + t2116 * t12606 / 2.0_f64);
    t94293
}
