//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta734 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2592;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta734<F: Float>(t11536: F, t4889: F, t1174: F, t15268: F, t15281: F, t11570: F, t12652: F, t1709: F, t44633: F, t11530: F, t15273: F, t11533: F, t457: F, t4936: F, t698: F, t15277: F, t3431: F, t15303: F, t11540: F, t11529: F, t4912: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52240, t52250, t52271, t52281, t52288, t52296, t52300) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2592::<F>(t11536, t4889, t1174, t15268, t15281, t11570, t12652, t1709, t44633, t11530, t15273, t11533);
        let (t52354, t52357, t52362, t52364, t52367) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2593::<F>(t1174, t457, t4936, t698, t15277, t3431, t15281, t15303, t11540, t4889, t11529, t4912);
    (t52240, t52250, t52271, t52281, t52288, t52296, t52300, t52354, t52357, t52362, t52364, t52367)
}
