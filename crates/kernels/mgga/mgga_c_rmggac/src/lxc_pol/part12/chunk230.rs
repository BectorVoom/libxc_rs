//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 230/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk230<F: Float>(t214: F, t7: F, t5: F, t12: F, t140: F, t953: F, t368: F, t956: F, t142: F, t36: F, t6: F, t954: F, t957: F) -> (F, F, F, F, F, F, F) {
    let t959 = t7 * t214;
    let t960 = t5 * t959;
    let t962 = F::new(1.0)/f64::sqrt(t12);
    let t963 = t962 * t140;
    let t964 = t963 * t953;
    let t966 = t368 * t956;
    let t969 = t142 * t6 * t36;
    let t971 = -F::new(0.57538888888888888889e0) * t954 + F::new(0.11507777777777777778e1) * t957 + F::new(0.40256666666666666667e0) * t960 + F::new(0.366775e-1) * t964 + F::new(0.73355e-1) * t966 + F::new(0.137975e0) * t969;
    (t959, t960, t963, t964, t966, t969, t971)
}
