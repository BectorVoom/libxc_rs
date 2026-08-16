//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1117/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1117(t22233: f64, t22293: f64, t22296: f64, t2196: f64, t3030: f64, t1171: f64, t6141: f64, t2256: f64, t3080: f64, t1189: f64, t6312: f64, t1235: f64, t5722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22800 = 0.20659e1_f64 * t22233;
    let t22811 = 0.104195e1_f64 * t22293;
    let t22812 = 0.104195e1_f64 * t22296;
    let t22820 = t3030 * t2196;
    let t22823 = t1171 * t6141;
    let t22826 = t3080 * t2256;
    let t22829 = t1189 * t6312;
    let t22919 = t1235 * t5722;
    (t22800, t22811, t22812, t22820, t22823, t22826, t22829, t22919)
}
