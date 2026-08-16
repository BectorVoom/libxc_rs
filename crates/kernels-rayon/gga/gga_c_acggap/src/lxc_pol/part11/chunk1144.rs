//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1144/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1144(t15386: f64, t31057: f64, t35700: f64, t3176: f64, t33953: f64, t13287: f64, t31443: f64, t31482: f64, t31484: f64, t31487: f64, t31489: f64, t31492: f64, t35668: f64, t35670: f64, t35673: f64, t35674: f64, t35676: f64, t35679: f64, t35683: f64, t35686: f64, t35691: f64, t35695: f64, t35698: f64) -> (f64, f64) {
    let t35702 = t31057 * t15386 * t35700;
    let t35703 = 0.94344276868812456204e-3_f64 * t35702;
    let t35704 = t33953 * t3176;
    let t35706 = t31443 * t13287 * t35704;
    let t35708 = 0.17149607247227894789e-1_f64 * t35668 + 0.85748036236139473945e-2_f64 * t35670 - t35673 - 0.68598428988911579156e-2_f64 * t35674 - 0.34299214494455789578e-2_f64 * t35676 + t35679 - 0.28582678745379824648e-3_f64 * t31482 - t35683 - 0.25724410870841842183e-2_f64 * t31484 - t35686 + t31487 / 96.0_f64 - 0.4584375e-1_f64 * t31489 - 0.916875e-1_f64 * t31492 + 0.94344276868812456204e-2_f64 * t35691 - 0.31448092289604152068e-2_f64 * t35695 + 0.47172138434406228102e-2_f64 * t35698 + t35703 + 0.42874018118069736972e-3_f64 * t35706;
    (t35704, t35708)
}
