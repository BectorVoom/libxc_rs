//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1326/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1326(t109: f64, t110517: f64, t110549: f64, t110580: f64, t110623: f64, t111: f64, t8240: f64, t110240: f64, t110253: f64, t12521: f64, t12524: f64, t12813: f64, t1401: f64, t1458: f64, t16524: f64, t16535: f64, t20173: f64, t2180: f64, t2319: f64, t29934: f64, t30009: f64, t30180: f64, t30250: f64, t30253: f64, t3938: f64, t3941: f64, t4072: f64, t45560: f64, t5371: f64, t5376: f64, t55405: f64, t66940: f64, t8143: f64, t8161: f64, t8230: f64, t8251: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t110626 = piecewise3(t110, 0.0_f64, t110517 + t110549 + t110580 + t110623);
    let t110631 = t8240 * t111;
    let t110655 = 0.135e2_f64 * t5371 * t29934 + 27.0_f64 * t16535 * t8230 + 27.0_f64 * t45560 * t8251 + 27.0_f64 * t3938 * t30180 + 0.135e2_f64 * t1401 * t110626 + 0.135e2_f64 * t110253 * t1458 + 27.0_f64 * t110631 * t2319 + 54.0_f64 * t110240 * t5376 + 54.0_f64 * t20173 * t30250 + 54.0_f64 * t20173 * t30253 + 54.0_f64 * t16524 * t30009 + 54.0_f64 * t12524 * t30250 + 27.0_f64 * t55405 * t2180 + 0.135e2_f64 * t12521 * t8230 + 0.135e2_f64 * t8161 * t12813 + 54.0_f64 * t66940 * t8251 + 54.0_f64 * t3941 * t8143 * t4072;
    (t110626, t110655)
}
