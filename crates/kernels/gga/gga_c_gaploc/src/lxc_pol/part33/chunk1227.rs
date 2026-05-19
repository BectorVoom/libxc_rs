//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1227/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1227<F: Float>(t32817: F, t2033: F, t2365: F, t27728: F, t24478: F, t7390: F, t22672: F, t2684: F, t3488: F, t10886: F, t28439: F, t10931: F, t23220: F, t32514: F) -> (F, F, F, F, F, F) {
    let t32818 = F::cast_from(0.51123901271894332902e0_f64) * t32817;
    let t32820 = t2033 * t2365 * t27728;
    let t32821 = F::cast_from(0.89376224879626066674e-1_f64) * t32820;
    let t32823 = t7390 * t2365 * t24478;
    let t32824 = F::cast_from(0.14896037479937677779e-1_f64) * t32823;
    let t32826 = t2684 * t22672 * t3488;
    let t32827 = F::cast_from(0.59644551483876721719e0_f64) * t32826;
    let t32828 = t10886 * t28439;
    let t32829 = F::cast_from(0.59584149919750711116e-1_f64) * t32828;
    let t32832 = F::cast_from(0.55213813373645879534e2_f64) * t23220 * t10931 * t32514;
    (t32818, t32821, t32824, t32827, t32829, t32832)
}
