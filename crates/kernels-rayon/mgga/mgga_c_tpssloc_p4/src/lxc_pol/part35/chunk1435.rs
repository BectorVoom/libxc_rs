//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1435/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1435(t5: f64, t25: f64, t265: f64, t394: f64, t108939: f64, t108983: f64, t109004: f64, t109025: f64, t112: f64, t106606: f64, t105830: f64, t1409: f64, t20217: f64, t2116: f64, t29507: f64, t40: f64, t5398: f64, t7992: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t109028 = piecewise3(t8, 0.0_f64, t108939 + t108983 + t109004 + t109025);
    let t109029 = t109028 * t112;
    let t109045 = piecewise3(t395, 0.0_f64, t106606);
    let t109055 = piecewise3(t115, t105830, t109045 * t40 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t29507 * t1409 + 3.0_f64 / 2.0_f64 * t7992 * t5398 + t2116 * t20217 / 2.0_f64);
    (t109029, t109055)
}
