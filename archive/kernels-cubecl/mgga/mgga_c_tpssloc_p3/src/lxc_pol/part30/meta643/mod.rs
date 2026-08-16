//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2053;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta643<F: Float>(t23562: F, t343: F, t88405: F, t1036: F, t25622: F, t14134: F, t6765: F, t1933: F, t23479: F, t88360: F, t88365: F, t25637: F, t984: F, t1014: F, t82654: F, t6722: F, t1409: F, t344: F, t1009: F, t6740: F, t23473: F, t3082: F, t7586: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t88407, t88415, t88422, t88425, t88428, t88430) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2053::<F>(t23562, t343, t88405, t1036, t25622, t14134, t6765, t1933, t23479, t88360, t88365, t25637, t984);
        let (t88431, t88440, t88449, t88451, t88453, t88479) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2054::<F>(t1014, t82654, t23479, t25637, t6722, t1409, t344, t1009, t6740, t23473, t3082, t7586);
    (t88407, t88415, t88422, t88425, t88428, t88430, t88431, t88440, t88449, t88451, t88453, t88479)
}
