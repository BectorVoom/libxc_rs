//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1808;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1809;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta461<F: Float>(t23218: F, t6553: F, t1880: F, t2553: F, t6554: F, t6552: F, t218: F, t23150: F, t212: F, t252: F, t23171: F, t23168: F, t6556: F, t22975: F, t22979: F, t23191: F, t23198: F, t23202: F, t23207: F, t23209: F, t23211: F, t23215: F, t259: F, t2597: F, t2713: F, t6632: F, t6663: F, t855: F, t6547: F, t6573: F, t214: F, t852: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23219, t23220, t23222, t23223, t23224, t23226, t23228) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1808::<F>(t23218, t6553, t1880, t2553, t6554, t6552, t218, t23150, t212, t252);
        let (t23229, t23231, t23232, t23234) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1809::<F>(t23228, t6554, t23171, t23168, t6556, t22975, t22979, t23191, t23198, t23202, t23207, t23209, t23211, t23215, t23220, t23224, t23226, t259, t2597, t2713, t6632, t6663, t855);
        let (t23235, t23236, t23237) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1810::<F>(t6547, t6573, t214, t852);
    (t23219, t23222, t23223, t23226, t23228, t23229, t23231, t23232, t23234, t23235, t23236, t23237)
}
