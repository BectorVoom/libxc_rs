//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1051/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1051(t27059: f64, t448: f64, t429: f64, t745: f64, t116: f64, t428: f64, t2849: f64, t371: f64, t26336: f64, t3086: f64, t8428: f64, t1113: f64, t8414: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27061 = 0.18781521737197933637e-2_f64 * t448 * t27059;
    let t27071 = t745 * t429;
    let t27074 = 5.0_f64 / 486.0_f64 * t428 * t116 * t27071;
    let t27082 = 1.0_f64 / t371 / t2849;
    let t27083 = t27082 * t26336;
    let t27100 = t3086 * t8428;
    let t27112 = t1113 * t8414;
    (t27061, t27071, t27074, t27082, t27083, t27100, t27112)
}
