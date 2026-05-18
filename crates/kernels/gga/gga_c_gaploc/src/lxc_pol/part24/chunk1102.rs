//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1102/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1102<F: Float>(t10914: F, t21504: F, t2365: F, t21784: F, t6111: F, t10928: F, t6574: F, t822: F, t123: F, t15499: F, t21503: F, t883: F) -> (F, F, F, F) {
    let t28633 = F::new(0.17875244975925213335e0) * t10914 * t2365 * t21504;
    let t28636 = F::new(0.59584149919750711116e-1) * t6111 * t2365 * t21784;
    let t28640 = t822 * t10928 * t6574;
    let t28641 = t15499 * t123;
    let t28645 = F::new(0.46011511144704899612e1) * t28640 * t28641 * t883 * t21503;
    (t28633, t28636, t28640, t28645)
}
