//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1009/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1009(t256: f64, t3610: f64, t2133: f64, t3553: f64, t2436: f64, t3724: f64, t10566: f64, t10568: f64, t10686: f64, t10688: f64, t10692: f64, t10693: f64, t10694: f64, t10697: f64, t10700: f64, t10702: f64, t10897: f64, t1364: f64, t1692: f64, t198: f64, t207: f64, t2116: f64, t2439: f64, t2440: f64, t3548: f64, t3552: f64, t3683: f64, t750: f64, t8012: f64, t821: f64, t8222: f64, t8225: f64, t823: f64) -> f64 {
    let t10901 = t256 * t3610;
    let t10905 = t3553 * t2133;
    let t10911 = t3724 * t2436;
    let t10918 = t10897 * t198 * t207 * t823 + 12.0_f64 * t10901 * t3552 * t750 - 2.0_f64 * t10911 * t1692 * t821 - 3.0_f64 * t1364 * t2439 * t8012 + 6.0_f64 * t2116 * t3548 * t3552 + 12.0_f64 * t2440 * t3552 * t3683 + 6.0_f64 * t10905 * t3552 + t10566 + t10568 - t10686 + t10688 + t10692 - t10693 + t10694 + t10697 + t10700 + t10702 + t8222 + t8225;
    t10918
}
