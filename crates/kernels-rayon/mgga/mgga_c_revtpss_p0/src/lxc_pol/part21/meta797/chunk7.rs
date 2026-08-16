//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2888/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2888(t1633: f64, t3012: f64, t11410: f64, t11450: f64, t11461: f64, t11467: f64, t11507: f64, t11521: f64, t15290: f64, t311: f64, t4673: f64, t4711: f64, t52207: f64, t52209: f64, t52211: f64, t52213: f64, t52216: f64, t52218: f64, t52221: f64, t52223: f64, t52226: f64, t52229: f64, t52405: f64, t52426: f64) -> f64 {
    let t52430 = t3012 * t1633;
    let t52433 = 0.6233709278045326953e3_f64 * t11507 * t4711 * t11467 + 0.11579025239058625248e4_f64 * t11450 * t4673 * t11410 + 0.10526802520742363173e2_f64 * t11461 * t15290 - 0.310907e-1_f64 * (t52405 + t52426) * t311 + 0.10526802520742363173e2_f64 * t52430 * t11521 + t52207 + t52209 - t52211 + t52213 - t52216 - t52218 - t52221 - t52223 - t52226 - t52229;
    t52433
}
