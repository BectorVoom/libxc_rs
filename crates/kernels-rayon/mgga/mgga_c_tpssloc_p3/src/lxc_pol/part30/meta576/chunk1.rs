//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1952/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1952(t28: f64, t265: f64, t504: f64, t28755: f64, t1409: f64, t1972: f64, t28802: f64, t52: f64, t5398: f64, t7664: f64, t28763: f64, t5161: f64, t7753: f64, t1983: f64, t113: f64, t1459: f64, t1980: f64, t24999: f64, t27993: f64, t27996: f64, t28020: f64, t28027: f64, t28029: f64, t28032: f64, t28034: f64, t28036: f64, t28038: f64, t28040: f64, t28042: f64, t28047: f64, t28240: f64, t510: f64, t5460: f64, t5494: f64, t574: f64, t6468: f64, t6517: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t28803 = piecewise3(t505, 0.0_f64, t28755);
    let t28810 = piecewise3(t401, t28802, t28803 * t52 / 2.0_f64 - t7664 * t1409 - t1972 * t5398 / 2.0_f64);
    let t28811 = t28763 + t28810;
    let t28813 = t7753 * t5161;
    let t28815 = 2.0_f64 * t1983 * t28813;
    let t28816 = -t113 * t28811 - 4.0_f64 * t1459 * t24999 + t1980 * t6468 - t27993 * t510 - 2.0_f64 * t27996 * t510 + t28020 * t574 - 4.0_f64 * t5460 * t6517 - 2.0_f64 * t5494 * t6517 - t28027 - t28029 - t28032 - t28034 - t28036 - t28038 - t28040 - t28042 - t28047 + t28240 - t28815;
    (t28803, t28811, t28813, t28816)
}
