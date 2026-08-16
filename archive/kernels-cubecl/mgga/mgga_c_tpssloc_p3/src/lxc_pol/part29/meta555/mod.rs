//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1956;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta555<F: Float>(t7359: F, t7999: F, t1186: F, t8077: F, t1222: F, t8043: F, t6729: F, t8027: F, t2140: F, t4965: F, t1202: F, t8048: F, t8049: F, t5017: F, t7337: F, t1207: F, t1218: F, t2136: F, t24675: F, t24681: F, t24690: F, t24704: F, t488: F, t4974: F, t5014: F, t5030: F, t7339: F, t7345: F) -> (F, F, F, F, F, F, F, F) {
        let (t27572, t27574, t27578, t27580, t27586, t27589) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1956::<F>(t7359, t7999, t1186, t8077, t1222, t8043, t6729, t8027, t2140, t4965, t1202, t8048);
        let (t27598, t27599, t27602) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1957::<F>(t1222, t8049, t5017, t7337, t1207, t1218, t2136, t24675, t24681, t24690, t24704, t27578, t27580, t27586, t27589, t488, t4974, t5014, t5030, t7339, t7345);
    (t27572, t27574, t27580, t27586, t27589, t27598, t27599, t27602)
}
