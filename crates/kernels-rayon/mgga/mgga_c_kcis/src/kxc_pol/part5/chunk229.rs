//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 229/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk229(t62: f64, t774: f64, t755: f64, t752: f64, t684: f64, t688: f64, t707: f64, t712: f64, t750: f64, t82: f64, t165: f64, t164: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t775 = t62 * t774;
    let t776 = t755 * t775;
    let t777 = t752 * t776;
    let t779 = t684 * t82 - 0.66725e-1_f64 * t688 * t707 - 0.13265555555555555555e-1_f64 * t712 + 0.99491666666666666664e-2_f64 * t750 - 0.99491666666666666664e-2_f64 * t777;
    let t780 = t779 * t165;
    let t781 = t164 * t164;
    (t775, t776, t777, t779, t780, t781)
}
