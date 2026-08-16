//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1951;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta534(t1863: f64, t26012: f64, t1410: f64, t2240: f64, t6505: f64, t7445: f64, t4017: f64, t71: f64, t12568: f64, t33: f64, t1409: f64, t22502: f64, t22505: f64, t22510: f64, t3961: f64, t3966: f64, t6500: f64, t67: f64, t1864: f64, t6509: f64, t7441: f64, t12571: f64, t6489: f64, t1860: f64, t1865: f64, t22544: f64, t22549: f64, t22551: f64, t26009: f64, t6486: f64, t6492: f64, t6506: f64, t6510: f64, t7428: f64, t7442: f64, t7446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26013, t26016, t26021, t26024, t26025, t26028, t26043) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1951(t1863, t26012, t1410, t2240, t6505, t7445, t4017, t71, t12568, t33, t1409, t22502, t22505, t22510, t3961, t3966, t6500);
        let (t26044, t26045, t26048, t26051, t26054) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1952(t26043, t67, t1864, t6509, t7441, t12571, t6489, t1860, t1865, t22544, t22549, t22551, t26009, t26013, t26016, t26021, t26025, t26028, t6486, t6492, t6506, t6510, t7428, t7442, t7446);
    (t26013, t26016, t26021, t26024, t26025, t26028, t26043, t26044, t26045, t26048, t26051, t26054)
}
