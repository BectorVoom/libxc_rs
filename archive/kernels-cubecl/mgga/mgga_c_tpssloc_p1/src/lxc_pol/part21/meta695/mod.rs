//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta695 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2522;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta695<F: Float>(t10469: F, t1603: F, t11058: F, t11045: F, t11064: F, t10236: F, t14165: F, t13831: F, t13847: F, t2986: F, t10254: F, t12648: F) -> (F, F, F, F, F, F, F) {
        let (t47840, t47841, t47853, t47857, t47887, t47907, t47919) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2522::<F>(t10469, t1603, t11058, t11045, t11064, t10236, t14165, t13831, t13847, t2986, t10254, t12648);
    (t47840, t47841, t47853, t47857, t47887, t47907, t47919)
}
