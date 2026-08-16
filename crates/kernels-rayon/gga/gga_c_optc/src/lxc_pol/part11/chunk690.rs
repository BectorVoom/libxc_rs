//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 690/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk690(t601: f64, t6825: f64, t518: f64, t622: f64, t84: f64, t596: f64, t120: f64, t2086: f64, t105: f64, t2156: f64, t635: f64, t127: f64, t2024: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6827 = 0.35089340384731224426e1_f64 * t601 * t6825;
    let t6838 = t518 * t622 * t84;
    let t6840 = 0.56969282336565386482e-3_f64 * t596 * t6838;
    let t6855 = t120 * t2086;
    let t6875 = t105 * t2156;
    let t6876 = t6875 * t635;
    let t6879 = t2024 * t127;
    (t6827, t6838, t6840, t6855, t6875, t6876, t6879)
}
