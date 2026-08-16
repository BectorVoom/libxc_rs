//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1808;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta543<F: Float>(t213: F, t6589: F, t9223: F, t6593: F, t23062: F, t23066: F, t22715: F, t229: F, t805: F, t1891: F, t192: F, t22690: F, t80881: F, t841: F, t244: F, t6546: F, t2606: F, t1878: F, t845: F, t2230: F, t23076: F, t23080: F, t200: F, t23075: F, t598: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81933, t81934, t81936, t81942, t81943, t81954) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1808::<F>(t213, t6589, t9223, t6593, t23062, t23066, t22715, t229, t805, t1891, t192, t22690, t80881, t841);
        let (t81956, t81957, t81959, t81962, t81964, t81968) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1809::<F>(t244, t6546, t2606, t1878, t845, t2230, t23076, t213, t23080, t200, t23075, t598);
    (t81933, t81934, t81936, t81942, t81943, t81954, t81956, t81957, t81959, t81962, t81964, t81968)
}
