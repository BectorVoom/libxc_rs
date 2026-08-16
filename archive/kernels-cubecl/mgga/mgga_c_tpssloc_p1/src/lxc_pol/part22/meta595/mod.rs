//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2115;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta595<F: Float>(t10469: F, t1603: F, t11058: F, t11045: F, t11064: F, t1597: F, t43052: F, t2986: F, t2990: F, t10189: F, t4540: F, t4542: F, t698: F, t973: F, t2403: F, t4392: F, t1553: F, t9709: F, t133: F, t135: F, t241: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47840, t47841, t47853, t47857, t48019, t48022, t48046, t48066) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2115::<F>(t10469, t1603, t11058, t11045, t11064, t1597, t43052, t2986, t2990, t10189, t4540, t4542, t698, t973);
        let (t48067, t48096, t48097, t48103, t48140) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2116::<F>(t48066, t2403, t4392, t1553, t9709, t133, t135, t241);
    (t47840, t47841, t47853, t47857, t48019, t48022, t48046, t48067, t48096, t48097, t48103, t48140)
}
