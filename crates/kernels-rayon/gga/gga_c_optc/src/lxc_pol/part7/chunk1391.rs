//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1391/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1391(t27705: f64, t3209: f64, t19: f64, t27175: f64, t1724: f64, t1107: f64, t3126: f64, t4457: f64, t4459: f64, t4464: f64, t4465: f64, t26936: f64, t4435: f64, t4437: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27706 = t3209 * t27705;
    let t27707 = t27175 * t19;
    let t27712 = t1724 * t27705;
    let t27719 = t1107 * t3126;
    let t27721 = t4457 * t27719 * t4459;
    let t27724 = t4464 * t27719 * t4465;
    let t27730 = t4435 * t26936 * t4437;
    (t27706, t27707, t27712, t27721, t27724, t27730)
}
