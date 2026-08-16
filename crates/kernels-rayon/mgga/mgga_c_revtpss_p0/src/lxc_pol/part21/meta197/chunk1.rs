//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1187/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1187(t324: f64, t4682: f64, t1626: f64, t964: f64, t1634: f64, t972: f64, t2848: f64, t2906: f64, t2994: f64, t3001: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t4599: f64, t4607: f64, t4615: f64, t4617: f64, t4620: f64, t4623: f64, t4626: f64, t4629: f64) -> (f64, f64, f64, f64) {
    let t4683 = t4682 * t324;
    let t4685 = t1626 * t964;
    let t4690 = t1634 * t972;
    let t4707 = -0.1294625e1_f64 * t4599 + 0.258925e1_f64 * t4607 + t2994 + 0.10064166666666666667e0_f64 * t2848 + 0.10064166666666666667e0_f64 * t4571 - 0.20128333333333333333e0_f64 * t4576 + 0.60385e0_f64 * t4581 - 0.301925e0_f64 * t4585 + 0.82524375e-1_f64 * t4615 + 0.16504875e0_f64 * t4617 + t3001 + 0.5519e-1_f64 * t2906 + 0.5519e-1_f64 * t4620 - 0.27595e-1_f64 * t4623 + 0.16557e0_f64 * t4626 - 0.82785e-1_f64 * t4629;
    (t4683, t4685, t4690, t4707)
}
