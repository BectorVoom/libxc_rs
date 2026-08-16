//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk847;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta206(t521: f64, t9861: f64, t17: f64, t1294: f64, t9494: f64, t1995: f64, t68: f64, t215: f64, t535: f64, t9569: f64, t1314: f64, t2559: f64, t795: f64, t9580: f64, t3749: f64, t9577: f64, t2566: f64, t3732: f64, t792: f64, t782: f64, t1365: f64, t154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12132, t12133, t12141, t12155, t12188, t12189) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk847(t521, t9861, t17, t1294, t9494, t1995, t68, t215, t535, t9569, t1314, t2559);
        let (t12194, t12196, t12199, t12202, t12211, t12214) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk848(t535, t795, t9580, t3749, t9577, t1314, t2566, t3732, t792, t782, t1365, t154);
    (t12132, t12133, t12141, t12155, t12188, t12189, t12194, t12196, t12199, t12202, t12211, t12214)
}
