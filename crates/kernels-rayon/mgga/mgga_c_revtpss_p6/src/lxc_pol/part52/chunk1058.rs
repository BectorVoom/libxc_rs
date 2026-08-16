//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1058/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1058(t7003: f64, t7359: f64, t7316: f64, t8698: f64, t2007: f64, t7373: f64, t196: f64, t197: f64, t7484: f64, t2035: f64, t7313: f64, t531: f64, t8713: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32619 = 2.0_f64 * t7359 * t7003;
    let t32620 = t8698 * t7316;
    let t32621 = t2007 * t7373;
    let t32626 = t7484 * t196 * t197;
    let t32627 = t32626 * t2035;
    let t32628 = t8698 * t7313;
    let t32629 = t531 * t8713;
    (t32619, t32620, t32621, t32626, t32627, t32628, t32629)
}
