//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1311/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1311<F: Float>(t12433: F, t1616: F, t687: F, t1615: F, t3855: F, t1617: F, t2011: F, t3859: F, t4915: F, t12329: F, t3483: F, t3537: F) -> (F, F, F, F, F) {
    let t38063 = F::new(4.0) * t1616 * t12433 * t687;
    let t38064 = t3855 * t1615;
    let t38066 = F::new(2.0) * t38064 * t1617;
    let t38069 = F::new(6.0) * t4915 * t3859 * t2011;
    let t38070 = t12329 * t2011;
    let t38073 = F::new(24.0) * t4915 * t3483 * t3537;
    (t38063, t38066, t38069, t38070, t38073)
}
