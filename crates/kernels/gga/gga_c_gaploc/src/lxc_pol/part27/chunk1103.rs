//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1103/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1103<F: Float>(t7344: F, t7803: F, t948: F, t20671: F, t22543: F, t22980: F, t21461: F, t2365: F, t7390: F, t10914: F, t21504: F, t21784: F, t6111: F) -> (F, F, F, F, F) {
    let t28569 = t7803 * t948 * t7344;
    let t28585 = F::new(0.17041300423964777634e0) * t22543 * t20671 * t22980;
    let t28593 = F::new(0.29792074959875355558e-1) * t7390 * t2365 * t21461;
    let t28633 = F::new(0.17875244975925213335e0) * t10914 * t2365 * t21504;
    let t28636 = F::new(0.59584149919750711116e-1) * t6111 * t2365 * t21784;
    (t28569, t28585, t28593, t28633, t28636)
}
