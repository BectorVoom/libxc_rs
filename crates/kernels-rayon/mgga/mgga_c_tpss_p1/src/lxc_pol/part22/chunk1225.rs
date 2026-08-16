//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1225/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1225(t30: f64, t259: f64, t379: f64, t18847: f64, t1819: f64, t18823: f64, t1992: f64, t45: f64, t581: f64, t5870: f64, t1006: f64, t1692: f64, t1812: f64, t18239: f64, t18247: f64, t18250: f64, t18254: f64, t18265: f64, t18268: f64, t18271: f64, t18728: f64, t18803: f64, t18807: f64, t18812: f64, t2439: f64, t2829: f64, t33: f64, t3552: f64, t5671: f64, t5678: f64, t5849: f64, t5853: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t18848 = piecewise3(t380, 0.0_f64, t18847);
    let t18855 = piecewise3(t120, t18823, t18848 * t45 / 2.0_f64 + t5870 * t581 + t1819 * t1992 / 2.0_f64);
    let t18887 = 3.0_f64 * t3552 * t1812 * t18239 + 3.0_f64 * t2439 * t5849 * t5671 - 3.0_f64 * t18728 * t18247 + 3.0_f64 * t2439 * t1812 * t18250 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t18254 + t1692 * t18803 * t33 / 2.0_f64 - t1692 * t18807 * t5678 + t1692 * t5849 * t1006 + t1692 * t18812 * t18265 - t1692 * t5853 * t18268 - t1692 * t5853 * t18271 / 2.0_f64 + t1692 * t1812 * t2829 / 2.0_f64;
    (t18848, t18855, t18887)
}
