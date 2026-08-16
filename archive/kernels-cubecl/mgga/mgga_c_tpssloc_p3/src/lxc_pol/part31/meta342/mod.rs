//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1250;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta342<F: Float>(t1561: F, t2885: F, t2860: F, t13550: F, t13563: F, t13644: F, t13602: F, t4446: F, t942: F, t1573: F, t2929: F, t13566: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14271, t14276, t14287, t14291, t14321, t14324, t14332, t14337, t14352, t14353) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1250::<F>(t1561, t2885, t2860, t13550, t13563, t13644, t13602, t4446, t942, t1573, t2929, t13566);
    (t14271, t14276, t14287, t14291, t14321, t14324, t14332, t14337, t14352, t14353)
}
