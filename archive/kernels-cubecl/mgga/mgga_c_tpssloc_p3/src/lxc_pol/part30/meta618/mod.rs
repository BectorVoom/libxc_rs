//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2017;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta618<F: Float>(t608: F, t9239: F, t22522: F, t2267: F, t614: F, t38: F, t9287: F, t835: F, t39054: F, t6489: F, t39063: F, t531: F, t6995: F) -> (F, F, F, F, F, F, F, F) {
        let (t83717, t83741, t83791, t83796, t83803, t83827, t83830, t83859) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2017::<F>(t608, t9239, t22522, t2267, t614, t38, t9287, t835, t39054, t6489, t39063, t531, t6995);
    (t83717, t83741, t83791, t83796, t83803, t83827, t83830, t83859)
}
