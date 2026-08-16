//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1940/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1940(t5: f64, t25: f64, t265: f64, t394: f64, t27326: f64, t27368: f64, t112: f64, t25882: f64, t1409: f64, t2116: f64, t25398: f64, t3966: f64, t40: f64, t607: f64, t7274: f64, t7992: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t27370 = piecewise3(t8, 0.0_f64, t27326 + t27368);
    let t27371 = t27370 * t112;
    let t27373 = piecewise3(t395, 0.0_f64, t25882);
    let t27380 = piecewise3(t115, t25398, t7274 * t1409 / 2.0_f64 + t2116 * t3966 / 2.0_f64 + t27373 * t40 / 2.0_f64 + t7992 * t607 / 2.0_f64);
    (t27370, t27371, t27373, t27380)
}
