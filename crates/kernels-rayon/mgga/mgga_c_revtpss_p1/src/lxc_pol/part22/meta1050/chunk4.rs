//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3699/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3699(t21007: f64, t3625: f64, t44425: f64, t12772: f64, t21222: f64, t5340: f64, t21101: f64, t3707: f64, t1252: f64, t12855: f64, t17222: f64, t17426: f64, t17454: f64, t17456: f64, t17633: f64, t1797: f64, t20797: f64, t20800: f64, t20956: f64, t20963: f64, t21020: f64, t21223: f64, t3626: f64, t3720: f64, t44252: f64, t44578: f64, t44664: f64, t5293: f64, t57707: f64, t59375: f64, t59401: f64) -> f64 {
    let t70064 = t3625 * t44425 * t21007;
    let t70076 = t5340 * t12772 * t21222;
    let t70082 = t3707 * t21101;
    let t70085 = -0.57165357490759649296e-3_f64 * t17426 * t21223 - 0.57165357490759649296e-3_f64 * t3625 * t3626 * t17633 * t21020 + 0.91464571985215438872e-2_f64 * t57707 * t17456 - 0.25724410870841842184e-2_f64 * t59401 * t20963 + 0.42874018118069736972e-3_f64 * t44664 * t20797 + 0.31758531939310916276e-3_f64 * t70064 + 0.25724410870841842183e-2_f64 * t44578 * t3720 * t20956 * t17454 - 0.85748036236139473944e-3_f64 * t12855 * t3720 * t20800 * t17454 + 0.6351706387862183255e-4_f64 * t44252 - 0.3811023832717309953e-3_f64 * t70076 - 0.22866142996303859718e-2_f64 * t5293 * t17222 + 0.42874018118069736972e-3_f64 * t59375 * t1797 + 0.14481890564325777821e-1_f64 * t70082 * t1252;
    t70085
}
