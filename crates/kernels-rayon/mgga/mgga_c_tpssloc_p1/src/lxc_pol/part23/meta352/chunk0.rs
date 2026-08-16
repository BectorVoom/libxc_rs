//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1147/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1147(t2374: f64, t39497: f64, t39500: f64, t39506: f64, t10108: f64, t257: f64, t68: f64, t233: f64, t9970: f64, t252: f64, t2632: f64, t10021: f64, t812: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40799 = 0.67471172535210825684e-1_f64 * t2374 * t39497;
    let t40801 = 0.86748650402413918736e-1_f64 * t2374 * t39500;
    let t40803 = 0.38527786510141256862e1_f64 * t2374 * t39506;
    let t40889 = 1.0_f64 / t10108 / t257;
    let t40890 = t68 * t40889;
    let t40931 = 1.0_f64 / t9970 / t233;
    let t40932 = t40931 * t252;
    let t40933 = t2632 * t2632;
    let t40965 = t812 * t841 * t10021;
    (t40799, t40801, t40803, t40890, t40931, t40932, t40933, t40965)
}
