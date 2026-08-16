//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 812/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk812(t2974: f64, t659: f64, t2331: f64, t946: f64, t2977: f64, t2971: f64, t251: f64, t2887: f64, t1075: f64, t237: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9702 = t659 * t2974;
    let t9708 = t2331 * t946;
    let t9710 = t659 * t2977;
    let t9712 = t659 * t2971;
    let t9714 = t251 * t2887;
    let t9725 = t237 * t1075 * t240;
    (t9702, t9708, t9710, t9712, t9714, t9725)
}
