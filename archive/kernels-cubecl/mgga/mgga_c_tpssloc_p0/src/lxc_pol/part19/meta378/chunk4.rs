//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1415/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1415<F: Float>(t11292: F, t1156: F, t1164: F, t43679: F, t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F) -> (F, F) {
    let t43924 = F::cast_from(0.14035736694323150897e2_f64) * t1164 * t11292 * t43679 * t1156;
    let t43936 = -F::cast_from(0.16481481481481481482e-1_f64) * t43748 - F::cast_from(0.13734567901234567901e-1_f64) * t43750 + F::cast_from(0.24722222222222222222e-1_f64) * t43780 + F::cast_from(0.49444444444444444445e-1_f64) * t43782 + F::cast_from(0.49444444444444444444e-1_f64) * t43784 - F::cast_from(0.74166666666666666668e-1_f64) * t43786 - F::cast_from(0.12361111111111111111e-1_f64) * t43788 + F::cast_from(0.12361111111111111111e0_f64) * t43794 - F::cast_from(0.22249999999999999999e0_f64) * t43798 + F::cast_from(0.2225e0_f64) * t43802 + F::cast_from(0.92708333333333333333e-2_f64) * t43806;
    (t43924, t43936)
}
