//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2063;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta529(t12379: f64, t3799: f64, t12384: f64, t3777: f64, t3795: f64, t12282: f64, t3809: f64, t12328: f64, t1333: f64, t1336: f64, t2690: f64, t3788: f64, t67: f64, t6924: f64, t246: f64, t12156: f64, t550: f64, t12012: f64, t12371: f64, t16398: f64, t12283: f64, t12426: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40128, t40131, t40138, t40139, t40145, t40159) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2063(t12379, t3799, t12384, t3777, t3795, t12282, t3809, t12328, t1333, t1336, t2690, t3788);
        let (t40160, t40167, t40168, t40169, t40178, t40188, t40190) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2064(t3795, t40159, t67, t6924, t246, t12156, t550, t12012, t12371, t16398, t12283, t12426);
    (t40128, t40131, t40138, t40139, t40145, t40160, t40167, t40168, t40169, t40178, t40188, t40190)
}
