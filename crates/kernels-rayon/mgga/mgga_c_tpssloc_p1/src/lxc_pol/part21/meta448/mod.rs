//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1997;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta448(t14731: f64, t3440: f64, t135: f64, t5045: f64, t1174: f64, t1222: f64, t4966: f64, t1215: f64, t1734: f64, t1089: f64, t475: f64, t607: f64, t3578: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15686, t15689, t15691, t15699, t15700, t15701, t15702) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1997(t14731, t3440, t135, t5045, t1174, t1222, t4966, t1215, t1734, t1089, t475, t607);
        let (t15703, t15704, t15708) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1998(t15700, t15702, t3578, t1215, t607, t475);
    (t15686, t15689, t15691, t15699, t15700, t15701, t15702, t15703, t15704, t15708)
}
