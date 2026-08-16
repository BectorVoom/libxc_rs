//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 912/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk912(t23460: f64, t23606: f64, t23609: f64, t29082: f64, t29085: f64, t29091: f64, t29097: f64, t29152: f64, t29155: f64, t29161: f64, t29164: f64, t29166: f64, t29168: f64, t29170: f64) -> f64 {
    let t29226 = -0.33218518518518518518e0_f64 * t29082 + 0.11958666666666666667e1_f64 * t29085 - 0.17938e1_f64 * t29091 - 0.29896666666666666667e0_f64 * t29097 - 0.73028148148148148146e-1_f64 * t29152 - 0.16431333333333333333e0_f64 * t29155 + 0.19931111111111111111e0_f64 * t23460 + 0.10954222222222222222e0_f64 * t23606 + 0.32862666666666666666e0_f64 * t23609 + 0.32862666666666666666e0_f64 * t29161 - 0.98587999999999999998e0_f64 * t29164 + 0.3071625e0_f64 * t29166 + 0.46074375e0_f64 * t29168 - 0.28483875e1_f64 * t29170;
    t29226
}
