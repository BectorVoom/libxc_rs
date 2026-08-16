//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 210/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk210(t163: f64, t80: f64, t81: f64, t867: f64, t869: f64, t874: f64, t88: f64) -> (f64, f64, f64) {
    let t877 = t80 * t81 * t163;
    let t879 = -0.632975e0_f64 * t867 - 0.29896666666666666667e0_f64 * t869 - 0.1023875e0_f64 * t874 - 0.82156666666666666667e-1_f64 * t877;
    let t880 = 1.0_f64 / t88;
    (t877, t879, t880)
}
