//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2229;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta543(t1216: f64, t5971: f64, t11668: f64, t1090: f64, t6225: f64, t3578: f64, t11697: f64, t6191: f64, t3577: f64, t248: f64, t3570: f64, t6219: f64, t1213: f64, t5979: f64, t5975: f64, t11678: f64, t11709: f64, t11734: f64, t1227: f64, t15438: f64, t15569: f64, t18342: f64, t18346: f64, t18357: f64, t18360: f64, t3490: f64, t4954: f64, t4984: f64, t5014: f64, t5019: f64, t6203: f64, t6227: f64, t6232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18363, t18364, t18367, t18368, t18371, t18372, t18375) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2229(t1216, t5971, t11668, t1090, t6225, t3578, t11697, t6191, t3577, t248, t3570, t6219);
        let (t18382, t18383, t18386, t18387, t18390) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2230(t1213, t18375, t1216, t5979, t3578, t5975, t11678, t11709, t11734, t1227, t15438, t15569, t18342, t18346, t18357, t18360, t18364, t18368, t18372, t3490, t3577, t4954, t4984, t5014, t5019, t6203, t6227, t6232);
    (t18363, t18364, t18367, t18368, t18371, t18375, t18382, t18383, t18386, t18387, t18390)
}
