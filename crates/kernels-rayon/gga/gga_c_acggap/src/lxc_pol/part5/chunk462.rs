//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 462/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk462(t1007: f64, t1034: f64, t1041: f64, t1173: f64, t1180: f64, t165: f64, t1745: f64, t1750: f64, t1755: f64, t976: f64, t979: f64, t983: f64, t989: f64, t995: f64) -> f64 {
    let t1758 = 0.42874018118069736972e-3_f64 * t165 * t1745 + t976 - t979 + t983 + t989 - t995 - t1007 + t1034 + t1041 + 0.17149607247227894789e-2_f64 * t1173 * t1750 - 0.85748036236139473944e-3_f64 * t1180 * t1755;
    t1758
}
