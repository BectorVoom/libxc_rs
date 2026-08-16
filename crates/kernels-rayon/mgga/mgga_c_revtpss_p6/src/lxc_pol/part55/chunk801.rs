//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 801/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk801(t552: f64, t8591: f64, t117: f64, t8460: f64, t572: f64, t136: f64, t8440: f64) -> (f64, f64, f64, f64) {
    let t8592 = t8591 * t552;
    let t8614 = t117 * t8460;
    let t8616 = 3.0_f64 * t572 * t8614;
    let t8621 = t136 * t8440;
    (t8592, t8614, t8616, t8621)
}
