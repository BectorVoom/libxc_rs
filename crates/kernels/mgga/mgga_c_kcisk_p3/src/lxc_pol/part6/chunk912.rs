//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 912/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk912<F: Float>(t23460: F, t23606: F, t23609: F, t29082: F, t29085: F, t29091: F, t29097: F, t29152: F, t29155: F, t29161: F, t29164: F, t29166: F, t29168: F, t29170: F) -> F {
    let t29226 = -F::cast_from(0.33218518518518518518e0_f64) * t29082 + F::cast_from(0.11958666666666666667e1_f64) * t29085 - F::cast_from(0.17938e1_f64) * t29091 - F::cast_from(0.29896666666666666667e0_f64) * t29097 - F::cast_from(0.73028148148148148146e-1_f64) * t29152 - F::cast_from(0.16431333333333333333e0_f64) * t29155 + F::cast_from(0.19931111111111111111e0_f64) * t23460 + F::cast_from(0.10954222222222222222e0_f64) * t23606 + F::cast_from(0.32862666666666666666e0_f64) * t23609 + F::cast_from(0.32862666666666666666e0_f64) * t29161 - F::cast_from(0.98587999999999999998e0_f64) * t29164 + F::cast_from(0.3071625e0_f64) * t29166 + F::cast_from(0.46074375e0_f64) * t29168 - F::cast_from(0.28483875e1_f64) * t29170;
    t29226
}
