//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2509/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2509(t14783: f64, t699: f64, t14786: f64, t14789: f64, t50946: f64, t50948: f64, t50950: f64, t50952: f64, t50954: f64, t50957: f64, t50961: f64, t50966: f64) -> (f64, f64, f64, f64) {
    let t50968 = t699 * t14783;
    let t50970 = t699 * t14786;
    let t50972 = t699 * t14789;
    let t50974 = 0.72462e1_f64 * t50946 + 0.80513333333333333334e0_f64 * t50948 + 0.40256666666666666667e0_f64 * t50950 + 0.20128333333333333333e0_f64 * t50952 + 0.12077e1_f64 * t50954 - 0.60384999999999999999e0_f64 * t50957 - 0.60384999999999999999e0_f64 * t50961 - 0.36230999999999999999e1_f64 * t50966 + 0.11038e0_f64 * t50968 + 0.55190000000000000001e-1_f64 * t50970 + 0.33114000000000000001e0_f64 * t50972;
    (t50968, t50970, t50972, t50974)
}
