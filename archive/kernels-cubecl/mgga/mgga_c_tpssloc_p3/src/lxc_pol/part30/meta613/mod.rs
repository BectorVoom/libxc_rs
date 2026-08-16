//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2010;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta613<F: Float>(t3082: F, t6759: F, t344: F, t607: F, t1009: F, t6740: F, t23509: F, t25651: F, t23563: F, t25650: F, t6750: F, t23482: F, t3: F, t23471: F, t10889: F, t23535: F, t3033: F, t1016: F, t3034: F, t1930: F, t23418: F, t3180: F, t10401: F, t23417: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82885, t82892, t82895, t82911, t82914, t82926) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2010::<F>(t3082, t6759, t344, t607, t1009, t6740, t23509, t25651, t23563, t25650, t6750, t23482, t3);
        let (t82943, t82956, t82986, t83008, t83015) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2011::<F>(t23471, t23482, t10889, t23535, t3033, t1016, t3034, t1930, t23418, t3180, t10401, t23417);
    (t82885, t82892, t82895, t82911, t82914, t82926, t82943, t82956, t82986, t83008, t83015)
}
