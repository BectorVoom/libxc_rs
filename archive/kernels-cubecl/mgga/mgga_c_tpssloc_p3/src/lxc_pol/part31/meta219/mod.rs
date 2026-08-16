//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta219<F: Float>(t5828: F, t977: F, t3003: F, t4384: F, t5718: F, t5721: F, t5724: F, t340: F, t343: F, t974: F, t1597: F, t2969: F, t2986: F, t4507: F, t4529: F, t5818: F, t5821: F, t5825: F, t973: F) -> (F, F, F, F, F, F, F, F) {
        let (t5829, t5836, t5838, t5839, t5842, t5844, t5845, t5848) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk952::<F>(t5828, t977, t3003, t4384, t5718, t5721, t5724, t340, t343, t974, t1597, t2969, t2986, t4507, t4529, t5818, t5821, t5825, t973);
    (t5829, t5836, t5838, t5839, t5842, t5844, t5845, t5848)
}
