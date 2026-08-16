//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1929/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1929(t15540: f64, t4582: f64, t12648: f64, t4987: f64, t13969: f64, t4983: f64, t3515: f64, t486: f64, t5011: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15541 = t4582 * t15540;
    let t15544 = t4987 * t12648;
    let t15545 = t4582 * t15544;
    let t15548 = t13969 * t4983;
    let t15550 = t3515 * t15548 / 2304.0_f64;
    let t15553 = t486 * t5011;
    (t15541, t15544, t15545, t15548, t15550, t15553)
}
