//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 949/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk949(t22213: f64, t13666: f64, t13668: f64, t13670: f64, t13887: f64, t9524: f64, t9542: f64, t9588: f64, t9598: f64, t9854: f64, t9857: f64, t9865: f64, t9868: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22928 = 0.17544670867903938621e1_f64 * t22213;
    let t22929 = 0.32530743900905219526e-1_f64 * t13666;
    let t22930 = 36.0_f64 * t13668;
    let t22931 = 96.0_f64 * t13670;
    let t22932 = 0.73245789224026180216e-3_f64 * t13887;
    let t22933 = -t9588 - t9524 + t9598 - t22928 + t22929 + t22930 + t22931 + t9542 - t9854 - t9857 + t9865 + t9868 + t22932;
    (t22928, t22929, t22930, t22931, t22932, t22933)
}
