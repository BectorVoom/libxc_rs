//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1195/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1195(t13287: f64, t2302: f64, t34823: f64, t8791: f64, t1761: f64, t30644: f64, t5807: f64, t7822: f64, t6153: f64, t6157: f64, t7647: f64, t1713: f64, t31491: f64, t7381: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40465 = t34823 * t13287 * t2302 * t8791;
    let t40467 = t30644 * t1761;
    let t40469 = t7822 * t5807;
    let t40472 = t7822 * t6153;
    let t40474 = t7647 * t6157;
    let t40477 = t31491 * t7381 * t1713;
    (t40465, t40467, t40469, t40472, t40474, t40477)
}
