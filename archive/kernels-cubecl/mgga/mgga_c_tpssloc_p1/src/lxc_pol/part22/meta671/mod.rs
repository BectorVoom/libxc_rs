//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta671<F: Float>(t10189: F, t5842: F, t2986: F, t2990: F, t13847: F, t13861: F, t17841: F, t2987: F, t13784: F, t17178: F, t5836: F, t17161: F) -> (F, F, F, F, F, F, F, F) {
        let (t61189, t61191, t61200, t61210, t61245, t61250, t61252, t61258) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2226::<F>(t10189, t5842, t2986, t2990, t13847, t13861, t17841, t2987, t13784, t17178, t5836, t17161);
    (t61189, t61191, t61200, t61210, t61245, t61250, t61252, t61258)
}
