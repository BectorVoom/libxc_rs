//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2042;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta605<F: Float>(t23272: F, t81651: F, t82074: F, t23204: F, t23218: F, t6562: F, t23171: F, t23228: F, t6572: F, t212: F, t6554: F, t852: F, t23030: F, t23253: F, t23241: F, t81640: F, t23273: F, t81591: F, t6555: F, t81573: F, t6563: F, t81597: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t82076, t82079, t82082, t82087) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2042::<F>(t23272, t81651, t82074, t23204, t23218, t6562, t23171, t23228, t6572, t212, t6554, t852);
        let (t82099, t82108, t82115, t82120, t82122) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2043::<F>(t23030, t23253, t23204, t23241, t81640, t23273, t81591, t23228, t6555, t81573, t6563, t81597);
    (t82076, t82079, t82082, t82087, t82099, t82108, t82115, t82120, t82122)
}
