//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1114/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1114<F: Float>(t10786: F, t1112: F, t1616: F, t12433: F, t687: F, t1615: F, t3855: F, t1617: F, t2011: F, t3859: F, t4915: F, t12329: F, t3483: F, t3537: F, t31767: F, t3480: F) -> (F, F, F, F, F, F, F, F) {
    let t38060 = 4.0 * t1616 * t1112 * t10786;
    let t38063 = 4.0 * t1616 * t12433 * t687;
    let t38064 = t3855 * t1615;
    let t38066 = 2.0 * t38064 * t1617;
    let t38069 = 6.0 * t4915 * t3859 * t2011;
    let t38070 = t12329 * t2011;
    let t38073 = 24.0 * t4915 * t3483 * t3537;
    let t38075 = 8.0 * t31767 * t3483;
    let t38077 = 2.0 * t3480 * t10786;
    (t38060, t38063, t38066, t38069, t38070, t38073, t38075, t38077)
}
