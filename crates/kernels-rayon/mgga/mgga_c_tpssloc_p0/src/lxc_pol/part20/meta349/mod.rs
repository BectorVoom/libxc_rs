//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1654;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta349(t12178: f64, t1380: f64, t3856: f64, t3901: f64, t215: f64, t535: f64, t9569: f64, t1314: f64, t2559: f64, t1317: f64, t795: f64, t9580: f64, t3749: f64, t9577: f64, t3726: f64, t3745: f64, t2566: f64, t3741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12179, t12181, t12188, t12189) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1654(t12178, t1380, t3856, t3901, t215, t535, t9569, t1314, t2559);
        let (t12190, t12194, t12196, t12197, t12199, t12200) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1655(t12189, t1317, t535, t795, t9580, t3749, t9577, t3726, t3745, t1314, t2566, t3741);
    (t12179, t12181, t12188, t12189, t12190, t12194, t12196, t12197, t12199, t12200)
}
