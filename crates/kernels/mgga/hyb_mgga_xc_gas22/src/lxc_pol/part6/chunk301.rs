//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 301/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk301<F: Float>(t439: F, t10: F, t1034: F, t14: F, t237: F, t800: F, t1035: F, t1037: F) -> (F, F, F, F) {
    let t1039 = F::sqrt(t439);
    let t1040 = t1039 * t10;
    let t1041 = t1040 * t1034;
    let t1044 = t237 * t14 * t800;
    let t1046 = -F::new(0.632975e0) * t1035 - F::cast_from(0.29896666666666666667e0_f64) * t1037 - F::new(0.1023875e0) * t1041 - F::cast_from(0.82156666666666666667e-1_f64) * t1044;
    (t1040, t1041, t1044, t1046)
}
