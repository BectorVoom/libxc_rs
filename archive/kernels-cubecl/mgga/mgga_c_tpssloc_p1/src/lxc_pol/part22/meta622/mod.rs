//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2155;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta622<F: Float>(t1174: F, t5045: F, t698: F, t3540: F, t4966: F, t11647: F, t1744: F, t3247: F, t475: F, t15032: F, t3576: F, t11713: F, t11716: F, t53081: F, t3032: F, t52434: F, t3505: F, t3514: F, t11835: F, t4889: F, t1725: F, t2402: F, t3506: F, t4979: F, t49850: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53271, t53273, t53274, t53298, t53322, t53336) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2155::<F>(t1174, t5045, t698, t3540, t4966, t11647, t1744, t3247, t475, t15032, t3576, t11713, t11716, t53081);
        let (t53372, t53399, t53434, t53440, t53452) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2156::<F>(t3032, t52434, t3505, t3514, t11835, t4889, t1174, t1725, t2402, t3506, t4979, t49850);
    (t53271, t53273, t53274, t53298, t53322, t53336, t53372, t53399, t53434, t53440, t53452)
}
