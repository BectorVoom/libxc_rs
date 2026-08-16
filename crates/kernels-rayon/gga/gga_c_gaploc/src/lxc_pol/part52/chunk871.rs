//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 871/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk871(t45529: f64, t2631: f64, t36515: f64, t787: f64, t10827: f64, t11053: f64, t9805: f64, t2679: f64, t3630: f64, t9796: f64, t11755: f64, t2028: f64, t2536: f64) -> (f64, f64, f64, f64, f64) {
    let t45530 = 0.29792074959875355558e-1_f64 * t45529;
    let t45536 = 0.17875244975925213335e2_f64 * t787 * t36515 * t2631;
    let t45542 = t9805 * t11053 * t10827;
    let t45543 = 0.11502877786176224903e1_f64 * t45542;
    let t45548 = t9796 * t3630 * t2679;
    let t45549 = 0.38342925953920749676e0_f64 * t45548;
    let t45553 = 0.39722766613167140743e-1_f64 * t787 * t2536 * t11755 * t2028;
    (t45530, t45536, t45543, t45549, t45553)
}
