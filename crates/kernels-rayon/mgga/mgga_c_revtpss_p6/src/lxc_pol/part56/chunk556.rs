//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 556/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk556(t373: f64, t4772: f64, t371: f64, t372: f64, t225: f64, t4746: f64, t366: f64, t4589: f64, t4592: f64, t4594: f64, t4597: f64, t4634: f64, t4638: f64, t4716: f64, t4718: f64, t4721: f64, t4723: f64, t4727: f64, t4731: f64, t4736: f64) -> (f64, f64, f64, f64) {
    let t4852 = t373 * t4772;
    let t4854 = t371 * t372 * t4852;
    let t4857 = t4746 * t225;
    let t4858 = t4857 * t366;
    let t4866 = -t4589 + t4592 + t4594 - t4597 + t4634 + t4638 + t4716 + t4718 - t4721 - t4723 + t4727 - t4731 - t4736;
    (t4854, t4857, t4858, t4866)
}
