//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1010/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1010(t2116: f64, t256: f64, t3724: f64, t823: f64, t10704: f64, t10706: f64, t10709: f64, t10712: f64, t10716: f64, t10719: f64, t10721: f64, t10724: f64, t10727: f64, t10731: f64, t1364: f64, t1692: f64, t198: f64, t2428: f64, t2439: f64, t2440: f64, t3610: f64, t3728: f64, t750: f64, t7979: f64, t7988: f64, t7992: f64, t8045: f64, t8231: f64, t8234: f64) -> f64 {
    let t10919 = t2116 * t256;
    let t10923 = t3724 * t823;
    let t10935 = 6.0_f64 * t10919 * t1364 * t198 + 6.0_f64 * t10923 * t2439 * t750 + 3.0_f64 * t1364 * t2439 * t8045 - t1692 * t2428 * t3728 + 6.0_f64 * t2439 * t2440 * t3610 + t10704 + t10706 + t10709 + t10712 + t10716 - t10719 + t10721 + t10724 + t10727 + t10731 + t7979 + t7988 + t7992 - t8231 - t8234;
    t10935
}
