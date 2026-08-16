//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1360/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1360(t33: f64, t259: f64, t479: f64, t66751: f64, t10353: f64, t1289: f64, t1826: f64, t18888: f64, t1992: f64, t20632: f64, t3431: f64, t57: f64, t581: f64, t5889: f64, t6393: f64, t66796: f64, t66833: f64, t66870: f64, t66897: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t66900 = piecewise3(t480, 0.0_f64, t66751);
    let t66912 = piecewise3(t386, t66796 + t66833 + t66870 + t66897, t66900 * t57 / 2.0_f64 - t20632 * t581 - t6393 * t1992 / 2.0_f64 - t18888 * t1289 / 2.0_f64 - t5889 * t3431 - t1826 * t10353 / 2.0_f64);
    t66912
}
