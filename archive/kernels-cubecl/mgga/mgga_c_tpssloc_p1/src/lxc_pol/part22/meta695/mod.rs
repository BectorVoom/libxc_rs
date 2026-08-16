//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta695 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2275;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta695<F: Float>(t1213: F, t18941: F, t248: F, t3570: F, t15730: F, t5019: F, t3508: F, t6218: F, t1215: F, t11721: F, t6224: F, t15594: F, t4993: F, t11692: F, t11697: F, t18396: F, t18400: F, t3577: F, t11678: F, t19001: F, t11818: F, t6219: F, t3036: F, t6163: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t65424, t65444, t65464, t65469, t65474, t65479) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2275::<F>(t1213, t18941, t248, t3570, t15730, t5019, t3508, t6218, t1215, t11721, t6224, t15594, t4993);
        let (t65482, t65485, t65506, t65528, t65539) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2276::<F>(t11692, t11697, t18396, t18400, t3577, t11678, t19001, t11818, t1213, t248, t6219, t3036, t6163);
    (t65424, t65444, t65464, t65469, t65474, t65479, t65482, t65485, t65506, t65528, t65539)
}
