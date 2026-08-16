//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 915/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk915(t45542: f64, t2679: f64, t3630: f64, t9796: f64, t11755: f64, t2028: f64, t2536: f64, t787: f64, t11763: f64, t13506: f64, t4673: f64, t6060: f64) -> (f64, f64, f64, f64, f64) {
    let t45543 = 0.11502877786176224903e1_f64 * t45542;
    let t45548 = t9796 * t3630 * t2679;
    let t45549 = 0.38342925953920749676e0_f64 * t45548;
    let t45553 = 0.39722766613167140743e-1_f64 * t787 * t2536 * t11755 * t2028;
    let t45557 = 0.39722766613167140743e-1_f64 * t787 * t2536 * t11763 * t2028;
    let t45560 = 0.14300195980740170667e1_f64 * t6060 * t4673 * t13506;
    (t45543, t45549, t45553, t45557, t45560)
}
