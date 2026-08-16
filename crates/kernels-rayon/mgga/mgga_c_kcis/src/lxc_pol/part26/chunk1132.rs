//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1132/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1132(t1610: f64, t1650: f64, t27584: f64, t4440: f64, t4314: f64, t531: f64, t1615: f64, t6159: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28747 = t1650 * t1610;
    let t28748 = t27584 * t28747;
    let t28749 = t4440 * t28748;
    let t28752 = t4314 * t531;
    let t28753 = t1650 * t1615;
    let t28754 = t28752 * t28753;
    let t28755 = t6159 * t28754;
    (t28747, t28748, t28749, t28752, t28753, t28754, t28755)
}
