//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1364;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1365;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta335<F: Float>(t584: F, t9212: F, t111: F, t4025: F, t1454: F, t2281: F, t4044: F, t626: F, t4068: F, t2341: F, t92: F, t100: F, t2349: F, t4098: F, t751: F, t172: F, t4095: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12603, t12604, t12725) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1364::<F>(t584, t9212, t111, t4025);
        let (t12747, t12750, t12752, t12774, t12795, t12850, t12858) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1365::<F>(t1454, t2281, t4044, t626, t4068, t2341, t92, t100, t2349, t4098, t751, t172, t4095);
    (t12603, t12604, t12725, t12747, t12750, t12752, t12774, t12795, t12850, t12858)
}
