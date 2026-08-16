//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2170;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2171;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta634(t212: f64, t5187: f64, t12225: f64, t2586: f64, t16100: f64, t782: f64, t16093: f64, t16097: f64, t2566: f64, t2559: f64, t5194: f64, t5198: f64, t12214: f64, t67: f64, t792: f64, t133: f64, t1799: f64, t40369: f64, t6600: f64, t131: f64, t205: f64, t40024: f64, t1336: f64, t242: f64, t40042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54668, t54670, t54676, t54701) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2170(t212, t5187, t12225, t2586, t16100, t782, t16093, t16097, t2566, t2559, t5194, t5198);
        let (t54702, t54718, t54725, t54728, t54744) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2171(t54701, t12214, t67, t792, t133, t1799, t40369, t6600, t131, t205, t40024, t1336, t242, t40042);
    (t54668, t54670, t54676, t54702, t54718, t54725, t54728, t54744)
}
