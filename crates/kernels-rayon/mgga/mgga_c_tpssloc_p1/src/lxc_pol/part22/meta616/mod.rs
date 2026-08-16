//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2145;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta616(t15394: f64, t1714: f64, t3439: f64, t3447: f64, t461: f64, t4724: f64, t697: f64, t11554: f64, t1706: f64, t11545: f64, t134: f64, t4899: f64, t4928: f64, t1174: f64, t1709: f64, t44633: f64, t11530: f64, t4889: f64, t50853: f64, t51039: f64, t51051: f64, t457: f64, t4936: f64, t698: f64, t11529: f64, t4912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52100, t52110, t52124, t52133, t52140) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2145(t15394, t1714, t3439, t3447, t461, t4724, t697, t11554, t1706, t11545, t134, t4899, t4928);
        let (t52281, t52288, t52313, t52339, t52343, t52355, t52367) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2146(t1174, t1709, t44633, t11530, t4889, t50853, t51039, t51051, t457, t4936, t698, t11529, t4912);
    (t52100, t52110, t52124, t52133, t52140, t52281, t52288, t52313, t52339, t52343, t52355, t52367)
}
