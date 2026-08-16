//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 586/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk586(t2874: f64, t2876: f64, t2882: f64, t2887: f64, t2892: f64, t2895: f64, t2897: f64, t2900: f64, t2904: f64, t2907: f64, t3477: f64, t1104: f64, t575: f64) -> (f64, f64) {
    let t3478 = -0.3475929712541504153e-2_f64 * t2874 + 0.20855578275249024918e-2_f64 * t2876 - 0.20855578275249024918e-2_f64 * t2882 - 0.69518594250830083059e-4_f64 * t2887 + 0.12360406057797588768e-3_f64 * t2892 + 0.20855578275249024918e-2_f64 * t2895 + 0.27517776890953574545e-3_f64 * t2897 - 0.20855578275249024918e-2_f64 * t2900 - 0.26319242435966565832e-3_f64 * t2904 + 0.60736713313768998073e-4_f64 * t2907 + t3477;
    let t3480 = t1104 * t575;
    (t3478, t3480)
}
