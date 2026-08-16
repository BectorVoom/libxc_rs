//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1849;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta518<F: Float>(t1863: F, t26012: F, t1410: F, t2240: F, t6505: F, t7445: F, t4017: F, t71: F, t12568: F, t33: F, t1409: F, t22502: F, t22505: F, t22510: F, t3961: F, t3966: F, t6500: F, t67: F, t1864: F, t6509: F, t7441: F, t12571: F, t6489: F, t1860: F, t1865: F, t22544: F, t22549: F, t22551: F, t26009: F, t6486: F, t6492: F, t6506: F, t6510: F, t7428: F, t7442: F, t7446: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26013, t26016, t26021, t26024, t26025, t26028, t26043) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1849::<F>(t1863, t26012, t1410, t2240, t6505, t7445, t4017, t71, t12568, t33, t1409, t22502, t22505, t22510, t3961, t3966, t6500);
        let (t26044, t26045, t26048, t26051, t26054) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1850::<F>(t26043, t67, t1864, t6509, t7441, t12571, t6489, t1860, t1865, t22544, t22549, t22551, t26009, t26013, t26016, t26021, t26025, t26028, t6486, t6492, t6506, t6510, t7428, t7442, t7446);
    (t26013, t26016, t26021, t26024, t26025, t26028, t26043, t26044, t26045, t26048, t26051, t26054)
}
