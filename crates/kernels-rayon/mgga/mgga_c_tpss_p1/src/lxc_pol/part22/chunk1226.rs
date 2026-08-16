//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1226/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1226(t33: f64, t259: f64, t479: f64, t18847: f64, t1826: f64, t18887: f64, t1992: f64, t57: f64, t581: f64, t5889: f64, t18855: f64, t116: f64, t5798: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t18888 = piecewise3(t480, 0.0_f64, t18847);
    let t18895 = piecewise3(t386, t18887, t18888 * t57 / 2.0_f64 - t5889 * t581 - t1826 * t1992 / 2.0_f64);
    let t18896 = t18855 + t18895;
    let t18898 = t5798 * t116;
    (t18888, t18896, t18898)
}
