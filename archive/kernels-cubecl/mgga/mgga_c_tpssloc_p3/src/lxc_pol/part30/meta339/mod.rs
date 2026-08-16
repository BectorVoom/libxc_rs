//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1372;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta339<F: Float>(t13176: F, t816: F, t1512: F, t9671: F, t2697: F, t4257: F, t2563: F, t4159: F, t4155: F, t9573: F, t2644: F, t820: F) -> (F, F, F, F, F, F) {
        let (t13177, t13182, t13190, t13202, t13208, t13222) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1372::<F>(t13176, t816, t1512, t9671, t2697, t4257, t2563, t4159, t4155, t9573, t2644, t820);
    (t13177, t13182, t13190, t13202, t13208, t13222)
}
