//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1411/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1411(t14767: f64, t2477: f64, t828: f64, t1544: f64, t2394: f64, t10698: f64, t10811: f64, t4462: f64, t4416: f64, t808: f64, t10886: f64, t2703: f64, t4458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14769 = t2477 * t828 * t14767;
    let t14772 = t1544 * t2394;
    let t14774 = t10698 * t828 * t14772;
    let t14777 = t10811 * t4462;
    let t14779 = t808 * t4416;
    let t14780 = t10886 * t14779;
    let t14783 = 7.0_f64 / 72.0_f64 * t2703 * t4458;
    (t14769, t14772, t14774, t14777, t14780, t14783)
}
