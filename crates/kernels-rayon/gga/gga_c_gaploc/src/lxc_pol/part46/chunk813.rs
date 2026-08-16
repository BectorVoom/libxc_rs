//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 813/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk813(t22629: f64, t825: f64, t9438: f64, t900: f64, t9624: f64, t10023: f64, t10032: f64, t2021: f64, t7372: f64, t2673: f64, t40848: f64, t41416: f64, t969: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41477 = t825 * t9438 * t22629;
    let t41511 = t900 * t9624;
    let t41512 = t10023 * t41511;
    let t41515 = t2021 * t10032 * t7372;
    let t41518 = t2673 * t900 * t40848;
    let t41528 = t825 * t969 * t41416;
    (t41477, t41511, t41512, t41515, t41518, t41528)
}
