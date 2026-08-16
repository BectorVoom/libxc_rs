//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1020/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1020(t11155: f64, t11185: f64, t11187: f64, t11191: f64, t11196: f64, t11198: f64, t11200: f64, t11207: f64, t11211: f64, t6161: f64, t6175: f64, t7950: f64, t7955: f64, t9782: f64, t9819: f64, t9826: f64) -> f64 {
    let t11260 = 0.142419375e1_f64 * t11185 - 0.28483875e1_f64 * t11187 + 0.1898925e1_f64 * t11191 - t6161 + 0.11958666666666666667e1_f64 * t7955 - 0.89690000000000000001e0_f64 * t9782 + 0.8969e0_f64 * t11155 - 0.76790625e-1_f64 * t11196 + 0.46074375e0_f64 * t11198 + 0.3071625e0_f64 * t11200 - t6175 + 0.82156666666666666666e0_f64 * t7950 - 0.49293999999999999999e0_f64 * t9819 - 0.49293999999999999999e0_f64 * t9826 + 0.73941e0_f64 * t11207 + 0.24647e0_f64 * t11211;
    t11260
}
