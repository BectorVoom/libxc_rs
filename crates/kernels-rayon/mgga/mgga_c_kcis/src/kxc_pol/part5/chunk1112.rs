//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1112/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1112(t18749: f64, t4600: f64, t14282: f64, t1727: f64, t3293: f64, t330: f64, t6352: f64, t829: f64, t3274: f64, t1045: f64, t6334: f64, t18677: f64, t4579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18750 = t4600 * t18749;
    let t18753 = t14282 * t1727;
    let t18754 = t3293 * t18753;
    let t18757 = t6352 * t330;
    let t18758 = t18757 * t829;
    let t18759 = t3274 * t18758;
    let t18763 = t3274 * t6334 * t1045;
    let t18766 = t4579 * t18677;
    (t18750, t18753, t18754, t18759, t18763, t18766)
}
