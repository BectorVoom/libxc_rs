//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3116/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3116(t1263: f64, t16750: f64, t1012: f64, t1042: f64, t1122: f64, t1222: f64, t1225: f64, t12787: f64, t12836: f64, t12956: f64, t13002: f64, t13008: f64, t17502: f64, t17605: f64, t17736: f64, t17737: f64, t3625: f64, t3626: f64, t3629: f64, t3711: f64, t49889: f64, t5046: f64, t5373: f64, t57083: f64, t57257: f64, t57258: f64, t57265: f64, t57271: f64, t57274: f64, t57275: f64, t57290: f64, t57292: f64, t57295: f64, t57297: f64, t57299: f64) -> f64 {
    let t57303 = t1263 * t16750;
    let t57308 = t57257 + 0.47637797908966374413e-3_f64 * t57258 + 0.14291339372689912324e-2_f64 * t17736 * t12787 * t5046 * t57083 + 0.25724410870841842184e-2_f64 * t57265 * t3626 * t17737 * t57083 - t57271 + t57274 - 0.42874018118069736972e-3_f64 * t3625 * t3626 * t57275 * t3629 + 0.22866142996303859718e-2_f64 * t17605 * t12836 - t1222 * t1012 * t1225 * t49889 / 288.0_f64 + t5373 * t13002 / 108.0_f64 + t5373 * t13008 / 18.0_f64 + t57290 / 108.0_f64 + t57292 / 54.0_f64 - t57295 / 288.0_f64 - 0.22866142996303859718e-2_f64 * t57297 + 0.42874018118069736972e-3_f64 * t57299 + 0.85748036236139473944e-3_f64 * t12956 * t17502 + 0.42874018118069736972e-3_f64 * t3711 * t1042 * t57303 * t1122;
    t57308
}
