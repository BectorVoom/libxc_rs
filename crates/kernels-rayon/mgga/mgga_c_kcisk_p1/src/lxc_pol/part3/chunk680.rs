//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 680/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk680(t10569: f64, t10570: f64, t10572: f64, t10574: f64, t10576: f64, t10579: f64, t10582: f64, t10587: f64, t10590: f64, t10595: f64, t10598: f64, t587: f64) -> f64 {
    let t10600 = -t10569 - 0.23744444444444444444e-1_f64 * t10570 + 0.11872222222222222222e-1_f64 * t10572 - 0.35616666666666666666e-1_f64 * t10574 + 0.17808333333333333333e-1_f64 * t10576 - 0.19787037037037037037e-1_f64 * t10579 + 0.71233333333333333332e-1_f64 * t10582 - 0.35616666666666666666e-1_f64 * t10587 - 0.10685e0_f64 * t10590 + 0.10685e0_f64 * t10595 - 0.17808333333333333333e-1_f64 * t10598;
    let t10602 = 0.62182e-1_f64 * t10600 * t587;
    t10602
}
