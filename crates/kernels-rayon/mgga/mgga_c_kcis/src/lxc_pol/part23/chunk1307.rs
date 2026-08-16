//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1307/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1307(t12844: f64, t27583: f64, t28748: f64, t27566: f64, t28720: f64, t3801: f64, t6159: f64, t6207: f64, t27567: f64, t99422: f64, t27569: f64, t7978: f64, t98709: f64, t98712: f64, t98715: f64, t98719: f64, t98725: f64, t99087: f64, t99166: f64, t99432: f64) -> (f64, f64, f64) {
    let t99556 = 0.7722800925925925926e-4_f64 * t27583 * t12844 * t28748;
    let t99565 = t28720 * t27566;
    let t99569 = t6159 * t6207 * t3801;
    let t99578 = 0.10306077835648148148e-4_f64 * t27567 * t99422;
    let t99585 = 0.30918233506944444444e-4_f64 * t99565 * t27569 - 0.30918233506944444444e-4_f64 * t27567 * t99569 - 0.17411041666666666666e-2_f64 * t98709 + 0.23214722222222222222e-2_f64 * t98712 + 0.11607361111111111111e-2_f64 * t98715 - 0.38691203703703703703e-3_f64 * t98719 + 0.11607361111111111111e-2_f64 * t98725 + t99578 + 0.13901041666666666667e-2_f64 * t27583 * t99432 - 0.13901041666666666667e-2_f64 * t7978 * t99087 - 0.34752604166666666667e-3_f64 * t7978 * t99166;
    (t99556, t99569, t99585)
}
