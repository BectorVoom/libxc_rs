//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta520<F: Float>(t1998: F, t5318: F, t214: F, t1985: F, t7740: F, t794: F, t6897: F, t1825: F, t22873: F, t552: F, t6604: F) -> (F, F, F, F, F, F, F) {
        let (t26432, t26433, t26434, t26436, t26437, t26442, t26446) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1852::<F>(t1998, t5318, t214, t1985, t7740, t794, t6897, t1825, t22873, t552, t6604);
    (t26432, t26433, t26434, t26436, t26437, t26442, t26446)
}
