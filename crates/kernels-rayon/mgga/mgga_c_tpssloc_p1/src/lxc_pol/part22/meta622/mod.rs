//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2155;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta622(t1174: f64, t5045: f64, t698: f64, t3540: f64, t4966: f64, t11647: f64, t1744: f64, t3247: f64, t475: f64, t15032: f64, t3576: f64, t11713: f64, t11716: f64, t53081: f64, t3032: f64, t52434: f64, t3505: f64, t3514: f64, t11835: f64, t4889: f64, t1725: f64, t2402: f64, t3506: f64, t4979: f64, t49850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53271, t53273, t53274, t53298, t53322, t53336) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2155(t1174, t5045, t698, t3540, t4966, t11647, t1744, t3247, t475, t15032, t3576, t11713, t11716, t53081);
        let (t53372, t53399, t53434, t53440, t53452) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2156(t3032, t52434, t3505, t3514, t11835, t4889, t1174, t1725, t2402, t3506, t4979, t49850);
    (t53271, t53273, t53274, t53298, t53322, t53336, t53372, t53399, t53434, t53440, t53452)
}
