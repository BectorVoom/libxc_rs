//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 758/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk758(t9725: f64, t250: f64, t253: f64, t3106: f64, t242: f64, t245: f64, t255: f64, t2984: f64, t929: f64, t244: f64, t260: f64, t2987: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9726 = 0.93011851851851851854e0_f64 * t9725;
    let t9728 = t250 * t3106 * t253;
    let t9729 = 0.36514074074074074075e0_f64 * t9728;
    let t9736 = 28.0_f64 / 27.0_f64 * t9725;
    let t9752 = 1.0_f64/pow_3_2(t242);
    let t9758 = 1.0_f64 / t245 / t255 / 4.0_f64;
    let t9767 = 1.0_f64 / t2984 / t929;
    let t9768 = t244 * t9767;
    let t9770 = 1.0_f64 / t2987 / t260;
    (t9726, t9728, t9729, t9736, t9752, t9758, t9768, t9770)
}
