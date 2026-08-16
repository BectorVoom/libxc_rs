//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 399/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk399(t377: f64, t704: f64, t706: f64, t1762: f64, t717: f64, t722: f64, t595: f64, t766: f64, t637: f64, t160: f64, t36: f64, t164: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1763 = t377 * t704;
    let t1764 = t1763 * t706;
    let t1766 = 0.21687162600603479684e-1_f64 * t1762 * t1764;
    let t1767 = t377 * t717;
    let t1768 = t1767 * t722;
    let t1770 = 0.32106488758451047386e0_f64 * t1762 * t1768;
    let t1771 = t595 * t766;
    let t1772 = t1771 * t637;
    let t1774 = t160 * t36;
    let t1776 = 132.0_f64 * t1774 * t164;
    (t1763, t1764, t1766, t1767, t1768, t1770, t1771, t1772, t1774, t1776)
}
