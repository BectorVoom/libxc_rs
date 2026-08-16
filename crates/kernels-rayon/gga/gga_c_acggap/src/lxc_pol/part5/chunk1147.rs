//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1147/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1147(t3379: f64, t5717: f64, t5722: f64, t1165: f64, t1889: f64, t3194: f64, t4210: f64, t3382: f64, t6396: f64, t6400: f64, t1131: f64, t1173: f64, t1180: f64, t1552: f64, t15610: f64, t15622: f64, t1748: f64, t1759: f64, t372: f64, t4298: f64, t4313: f64, t4680: f64, t5715: f64, t6151: f64, t6258: f64, t6270: f64) -> f64 {
    let t20602 = t3379 * t5717;
    let t20618 = t3379 * t5722;
    let t20627 = t3194 * t1165 * t1889 * t4210;
    let t20629 = t3382 * t6396;
    let t20631 = t3382 * t6400;
    let t20642 = -0.34299214494455789578e-2_f64 * t15610 - 0.68598428988911579156e-2_f64 * t20602 - 0.68598428988911579156e-2_f64 * t1173 * t1165 * t4298 * t5715 - 0.68598428988911579156e-2_f64 * t1173 * t1165 * t1552 * t6258 * t372 - 0.34299214494455789578e-2_f64 * t1173 * t1165 * t1552 * t1748 * t1131 + 0.34299214494455789578e-2_f64 * t20618 - 0.51448821741683684366e-2_f64 * t1180 * t1165 * t4313 * t6151 * t372 - 0.17149607247227894789e-2_f64 * t20627 + 0.34299214494455789578e-2_f64 * t20629 - 0.17149607247227894789e-2_f64 * t20631 + 0.68598428988911579156e-2_f64 * t1173 * t4680 * t6270 - 0.25724410870841842183e-2_f64 * t1180 * t1165 * t4313 * t1759 * t1131 - 0.68598428988911579156e-2_f64 * t15622;
    t20642
}
