//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta584<F: Float>(t11018: F, t225: F, t11016: F, t11064: F, t42332: F, t11058: F, t3185: F, t42741: F, t1014: F, t42340: F, t42341: F, t3127: F) -> (F, F, F, F, F, F, F) {
        let (t43431, t43440, t43470, t43473, t43480, t43503, t43515) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2154::<F>(t11018, t225, t11016, t11064, t42332, t11058, t3185, t42741, t1014, t42340, t42341, t3127);
    (t43431, t43440, t43470, t43473, t43480, t43503, t43515)
}
