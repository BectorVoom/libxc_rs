//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1304/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1304(t13364: f64, t17173: f64, t322: f64, t5615: f64, t8790: f64, t1026: f64, t1165: f64, t1173: f64, t1181: f64, t1203: f64, t13286: f64, t13287: f64, t13671: f64, t1531: f64, t1567: f64, t17139: f64, t1859: f64, t18788: f64, t18805: f64, t18808: f64, t18810: f64, t18812: f64, t1884: f64, t23234: f64, t3176: f64, t3196: f64, t3266: f64, t386: f64, t418: f64, t5616: f64, t5679: f64) -> f64 {
    let t24246 = t17173 * t13364 * t8790 * t5615 * t322;
    let t24277 = -0.34299214494455789578e-2_f64 * t24246 + 0.85748036236139473944e-2_f64 * t418 * t1026 * t3266 * t1884 - 0.85748036236139473944e-3_f64 * t418 * t386 * t5679 * t1203 + 0.13719685797782315831e-1_f64 * t18788 + 0.25724410870841842183e-2_f64 * t1531 * t1165 * t23234 * t13671 + 0.34299214494455789578e-2_f64 * t18805 - 0.34299214494455789578e-1_f64 * t17139 * t13364 * t1859 * t3196 - 0.13719685797782315831e-1_f64 * t13286 * t13287 * t1859 * t3176 - 0.68598428988911579156e-2_f64 * t18808 + 0.34299214494455789577e-2_f64 * t18810 - 0.16006300097412701803e-1_f64 * t18812 + 0.68598428988911579156e-2_f64 * t1173 * t1181 * t1567 * t5616;
    t24277
}
