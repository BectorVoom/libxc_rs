//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta608<F: Float>(t23197: F, t6547: F, t23257: F, t6562: F, t794: F, t23012: F, t6568: F, t225: F, t23211: F, t23205: F, t82038: F, t23242: F, t81979: F) -> (F, F, F, F, F, F) {
        let (t82230, t82236, t82259, t82287, t82294, t82296) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2047::<F>(t23197, t6547, t23257, t6562, t794, t23012, t6568, t225, t23211, t23205, t82038, t23242, t81979);
    (t82230, t82236, t82259, t82287, t82294, t82296)
}
