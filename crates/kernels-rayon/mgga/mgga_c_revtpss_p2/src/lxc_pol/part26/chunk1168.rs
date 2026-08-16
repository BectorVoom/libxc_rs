//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1168/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1168(t2439: f64, t7398: f64, t780: f64, t785: f64, t93134: f64, t95546: f64, t26435: f64, t9303: f64, t26440: f64, t686: f64, t72: f64, t25375: f64) -> (f64, f64, f64, f64, f64) {
    let t95562 = t2439 * t785 * t7398 * t780;
    let t95567 = 0.43639970290213137151e-3_f64 * t93134 * t95546;
    let t95569 = 0.26019841438354088051e-2_f64 * t9303 * t26435;
    let t95571 = t26440 * t72 * t686;
    let t95572 = t25375 * t95571;
    (t95562, t95567, t95569, t95571, t95572)
}
