//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta378<F: Float>(t10469: F, t1603: F, t11058: F, t11045: F, t11064: F, t1597: F, t43052: F, t1553: F, t9709: F, t13797: F, t13783: F, t1599: F, t2402: F, t973: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47840, t47841, t47853, t47857, t48019, t48103, t48221, t48279, t48336) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1180::<F>(t10469, t1603, t11058, t11045, t11064, t1597, t43052, t1553, t9709, t13797, t13783, t1599, t2402, t973);
    (t47840, t47841, t47853, t47857, t48019, t48103, t48221, t48279, t48336)
}
