//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1227/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1227(t32817: f64, t2033: f64, t2365: f64, t27728: f64, t24478: f64, t7390: f64, t22672: f64, t2684: f64, t3488: f64, t10886: f64, t28439: f64, t10931: f64, t23220: f64, t32514: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32818 = 0.51123901271894332902e0_f64 * t32817;
    let t32820 = t2033 * t2365 * t27728;
    let t32821 = 0.89376224879626066674e-1_f64 * t32820;
    let t32823 = t7390 * t2365 * t24478;
    let t32824 = 0.14896037479937677779e-1_f64 * t32823;
    let t32826 = t2684 * t22672 * t3488;
    let t32827 = 0.59644551483876721719e0_f64 * t32826;
    let t32828 = t10886 * t28439;
    let t32829 = 0.59584149919750711116e-1_f64 * t32828;
    let t32832 = 0.55213813373645879534e2_f64 * t23220 * t10931 * t32514;
    (t32818, t32821, t32824, t32827, t32829, t32832)
}
