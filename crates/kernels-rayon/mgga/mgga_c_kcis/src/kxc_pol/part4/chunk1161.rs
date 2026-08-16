//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1161/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1161(t14717: f64, t3338: f64, t5046: f64, t3334: f64, t5083: f64, t1797: f64, t3429: f64, t1200: f64, t5169: f64, t14595: f64, t3438: f64, t3437: f64) -> (f64, f64, f64, f64, f64) {
    let t14726 = t3338 * t14717;
    let t14727 = t5046 * t14726;
    let t14729 = t5083 * t3334;
    let t14731 = t1797 * t3429;
    let t14733 = t5169 * t1200;
    let t14735 = t3438 * t14595;
    let t14736 = t3437 * t14735;
    (t14727, t14729, t14731, t14733, t14736)
}
