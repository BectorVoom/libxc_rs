//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta335<F: Float>(t1011: F, t11812: F, t1212: F, t486: F, t676: F, t1216: F, t248: F, t1213: F, t1226: F, t3566: F, t11552: F, t221: F) -> (F, F, F, F, F, F, F) {
        let (t11813, t11814, t11818, t11820, t11821, t11825, t11832) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1395::<F>(t1011, t11812, t1212, t486, t676, t1216, t248, t1213, t1226, t3566, t11552, t221);
    (t11813, t11814, t11818, t11820, t11821, t11825, t11832)
}
