//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta291<F: Float>(t2697: F, t2703: F, t842: F, t9612: F, t2617: F, t2696: F, t849: F, t232: F, t2553: F, t2614: F, t838: F, t2693: F, t809: F) -> (F, F, F, F, F, F, F) {
        let (t9988, t9990, t9993, t9994, t10007, t10012, t10014) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1349::<F>(t2697, t2703, t842, t9612, t2617, t2696, t849, t232, t2553, t2614, t838, t2693, t809);
    (t9988, t9990, t9993, t9994, t10007, t10012, t10014)
}
