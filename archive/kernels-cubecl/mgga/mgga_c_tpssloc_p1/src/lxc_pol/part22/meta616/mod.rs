//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2145;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta616<F: Float>(t15394: F, t1714: F, t3439: F, t3447: F, t461: F, t4724: F, t697: F, t11554: F, t1706: F, t11545: F, t134: F, t4899: F, t4928: F, t1174: F, t1709: F, t44633: F, t11530: F, t4889: F, t50853: F, t51039: F, t51051: F, t457: F, t4936: F, t698: F, t11529: F, t4912: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52100, t52110, t52124, t52133, t52140) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2145::<F>(t15394, t1714, t3439, t3447, t461, t4724, t697, t11554, t1706, t11545, t134, t4899, t4928);
        let (t52281, t52288, t52313, t52339, t52343, t52355, t52367) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2146::<F>(t1174, t1709, t44633, t11530, t4889, t50853, t51039, t51051, t457, t4936, t698, t11529, t4912);
    (t52100, t52110, t52124, t52133, t52140, t52281, t52288, t52313, t52339, t52343, t52355, t52367)
}
