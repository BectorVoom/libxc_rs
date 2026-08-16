//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2012;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta614<F: Float>(t3186: F, t83015: F, t3158: F, t6712: F, t10383: F, t1926: F, t10948: F, t23536: F, t10472: F, t10474: F, t10478: F, t23535: F, sigma0: F, t23540: F, t6753: F, t10375: F, t1942: F, t23488: F, t23509: F, t23508: F, t6721: F, t6741: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t83016, t83025, t83028, t83043, t83054, t83058) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2012::<F>(t3186, t83015, t3158, t6712, t10383, t1926, t10948, t23536, t10472, t10474, t10478, t23535, sigma0);
        let (t83061, t83065, t83080, t83117, t83121) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2013::<F>(t10948, t23540, t10472, t10478, t6753, t10375, t1942, t23488, t23509, t23508, t6721, t6741);
    (t83016, t83025, t83028, t83043, t83054, t83058, t83061, t83065, t83080, t83117, t83121)
}
