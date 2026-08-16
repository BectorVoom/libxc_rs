//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 831/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk831(t29560: f64, t1932: f64, t2133: f64, t7573: f64, t8027: f64, t1737: f64, t2136: f64, t24681: f64, t24704: f64, t27578: f64, t27592: f64, t27599: f64, t27609: f64, t27614: f64, t6203: f64, t6211: f64, t7345: f64) -> f64 {
    let t29561 = 1.0_f64 / t29560;
    let t29562 = t29561 * t1932;
    let t29563 = t29562 * t2133;
    let t29569 = t8027 * t7573;
    let t29580 = -t24681 + 0.72670960969452703541e-2_f64 * t29563 * t2136 + t27578 / 1152.0_f64 - t7345 * t6211 / 1152.0_f64 + 0.16149102437656156342e-2_f64 * t29569 * t2136 - t24704 - t27592 / 216.0_f64 - 0.20186378047070195428e-3_f64 * t27609 - t27599 * t1737 / 144.0_f64 + t27614 * t1737 / 768.0_f64 + 5.0_f64 / 6912.0_f64 * t7345 * t6203;
    t29580
}
