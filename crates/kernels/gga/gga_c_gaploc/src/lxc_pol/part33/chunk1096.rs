//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1096/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1096<F: Float>(t32820: F, t2365: F, t24478: F, t7390: F, t22672: F, t2684: F, t3488: F, t10886: F, t28439: F, t10931: F, t23220: F, t32514: F, t7572: F, t7573: F, t11054: F, t28073: F) -> (F, F, F, F, F, F, F) {
    let t32821 = 0.89376224879626066674e-1 * t32820;
    let t32823 = t7390 * t2365 * t24478;
    let t32824 = 0.14896037479937677779e-1 * t32823;
    let t32826 = t2684 * t22672 * t3488;
    let t32827 = 0.59644551483876721719e0 * t32826;
    let t32828 = t10886 * t28439;
    let t32829 = 0.59584149919750711116e-1 * t32828;
    let t32832 = 0.55213813373645879534e2 * t23220 * t10931 * t32514;
    let t32835 = 0.12423108009070322895e3 * t7572 * t7573 * t32514;
    let t32838 = t28073 * t11054;
    (t32821, t32824, t32827, t32829, t32832, t32835, t32838)
}
