//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 779/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk779(t5396: f64, t760: f64, t755: f64, t1973: f64, t5374: f64, t5400: f64, t10570: f64, t10572: f64, t10574: f64, t10576: f64, t10587: f64, t10595: f64, t10607: f64, t10610: f64, t10613: f64, t10615: f64, t10617: f64, t10619: f64, t10623: f64, t10626: f64) -> (f64, f64, f64, f64) {
    let t12017 = 1.0_f64 / t5396 / t760;
    let t12018 = t755 * t12017;
    let t12019 = t5374 * t1973;
    let t12020 = t12019 * t5400;
    let t12037 = -0.41678000000000000001e0_f64 * t10607 + 0.20839e0_f64 * t10610 - 0.62517e0_f64 * t10613 - 0.34731666666666666667e0_f64 * t10615 + 0.20839e0_f64 * t10617 + 0.69463333333333333335e-1_f64 * t10619 - 0.46308888888888888889e-1_f64 * t10623 - 0.104195e0_f64 * t10626 - 0.103295e1_f64 * t10587 + 0.309885e1_f64 * t10595 - 0.68863333333333333332e0_f64 * t10570 + 0.34431666666666666666e0_f64 * t10572 - 0.103295e1_f64 * t10574 + 0.51647499999999999999e0_f64 * t10576;
    (t12018, t12019, t12020, t12037)
}
