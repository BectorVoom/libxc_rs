//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1029/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1029(t20820: f64, t7877: f64, t30158: f64, t425: f64, t2083: f64, t7764: f64, t13263: f64, t19100: f64, t25590: f64, t25601: f64, t25609: f64, t30569: f64, t30572: f64, t30592: f64, t30595: f64, t30599: f64, t30603: f64, t30608: f64, t30610: f64, t30617: f64) -> (f64, f64, f64, f64) {
    let t30900 = t20820 * t7877;
    let t30909 = t425 * t30158;
    let t30916 = t2083 * t7764;
    let t30938 = 0.14865e-1_f64 * t30617 - 0.2973e-1_f64 * t30610 + 0.1982e-1_f64 * t30608 - t13263 - 0.55033333333333333332e-2_f64 * t19100 + 0.27516666666666666666e-2_f64 * t25590 - 0.82549999999999999999e-2_f64 * t25601 + 0.41274999999999999999e-2_f64 * t25609 - 0.45861111111111111112e-2_f64 * t30592 + 0.1651e-1_f64 * t30595 - 0.82550000000000000001e-2_f64 * t30569 - 0.24765e-1_f64 * t30599 + 0.24765e-1_f64 * t30572 - 0.41275e-2_f64 * t30603;
    (t30900, t30909, t30916, t30938)
}
