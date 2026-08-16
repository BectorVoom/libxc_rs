//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta695 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2275;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta695(t1213: f64, t18941: f64, t248: f64, t3570: f64, t15730: f64, t5019: f64, t3508: f64, t6218: f64, t1215: f64, t11721: f64, t6224: f64, t15594: f64, t4993: f64, t11692: f64, t11697: f64, t18396: f64, t18400: f64, t3577: f64, t11678: f64, t19001: f64, t11818: f64, t6219: f64, t3036: f64, t6163: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65424, t65444, t65464, t65469, t65474, t65479) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2275(t1213, t18941, t248, t3570, t15730, t5019, t3508, t6218, t1215, t11721, t6224, t15594, t4993);
        let (t65482, t65485, t65506, t65528, t65539) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2276(t11692, t11697, t18396, t18400, t3577, t11678, t19001, t11818, t1213, t248, t6219, t3036, t6163);
    (t65424, t65444, t65464, t65469, t65474, t65479, t65482, t65485, t65506, t65528, t65539)
}
