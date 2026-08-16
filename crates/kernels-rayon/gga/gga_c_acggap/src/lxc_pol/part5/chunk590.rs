//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 590/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk590(t1152: f64, t3621: f64, t1140: f64, t1156: f64, t1133: f64, t1117: f64, t1137: f64, t1121: f64, t107: f64, t2607: f64, t2690: f64, t4: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3622 = t3621 * t1152;
    let t3624 = t1140 * t1156;
    let t3634 = t1140 * t1133;
    let t3636 = t1137 * t1117;
    let t3638 = t1140 * t1121;
    let t3644 = -0.12962962962962962963e0_f64 * t4 * t2607 * t107 - 0.40124259259259259261e-1_f64 * t2690;
    (t3622, t3624, t3634, t3636, t3638, t3644)
}
