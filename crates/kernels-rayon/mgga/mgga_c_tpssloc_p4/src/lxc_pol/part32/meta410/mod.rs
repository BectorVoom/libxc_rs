//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1580;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1581;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1582;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta410(t17691: f64, t4987: f64, t4582: f64, t15654: f64, t17686: f64, t248: f64, t3570: f64, t6225: f64, t3506: f64, t1735: f64, t4733: f64, t3578: f64, t1216: f64, t5971: f64, t11668: f64, t1090: f64, t11697: f64, t6191: f64, t3577: f64, t6219: f64, t1213: f64, t5979: f64, t5975: f64, t11678: f64, t11709: f64, t11734: f64, t1227: f64, t15438: f64, t15569: f64, t3490: f64, t4954: f64, t4984: f64, t5014: f64, t5019: f64, t6203: f64, t6227: f64, t6232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18342, t18346, t18356, t18357, t18360) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1580(t17691, t4987, t4582, t15654, t17686, t248, t3570, t6225, t3506, t1735, t4733, t3578);
        let (t18364, t18368, t18371, t18372, t18375) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1581(t1216, t5971, t11668, t1090, t6225, t3578, t11697, t6191, t3577, t248, t3570, t6219);
        let (t18383, t18387, t18390) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1582(t1213, t18375, t1216, t5979, t3578, t5975, t11678, t11709, t11734, t1227, t15438, t15569, t18342, t18346, t18357, t18360, t18364, t18368, t18372, t3490, t3577, t4954, t4984, t5014, t5019, t6203, t6227, t6232);
    (t18342, t18346, t18356, t18360, t18364, t18368, t18371, t18375, t18383, t18387, t18390)
}
