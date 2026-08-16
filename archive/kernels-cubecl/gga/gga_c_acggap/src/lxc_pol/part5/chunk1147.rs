//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1147/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1147<F: Float>(t3379: F, t5717: F, t5722: F, t1165: F, t1889: F, t3194: F, t4210: F, t3382: F, t6396: F, t6400: F, t1131: F, t1173: F, t1180: F, t1552: F, t15610: F, t15622: F, t1748: F, t1759: F, t372: F, t4298: F, t4313: F, t4680: F, t5715: F, t6151: F, t6258: F, t6270: F) -> F {
    let t20602 = t3379 * t5717;
    let t20618 = t3379 * t5722;
    let t20627 = t3194 * t1165 * t1889 * t4210;
    let t20629 = t3382 * t6396;
    let t20631 = t3382 * t6400;
    let t20642 = -F::cast_from(0.34299214494455789578e-2_f64) * t15610 - F::cast_from(0.68598428988911579156e-2_f64) * t20602 - F::cast_from(0.68598428988911579156e-2_f64) * t1173 * t1165 * t4298 * t5715 - F::cast_from(0.68598428988911579156e-2_f64) * t1173 * t1165 * t1552 * t6258 * t372 - F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t1165 * t1552 * t1748 * t1131 + F::cast_from(0.34299214494455789578e-2_f64) * t20618 - F::cast_from(0.51448821741683684366e-2_f64) * t1180 * t1165 * t4313 * t6151 * t372 - F::cast_from(0.17149607247227894789e-2_f64) * t20627 + F::cast_from(0.34299214494455789578e-2_f64) * t20629 - F::cast_from(0.17149607247227894789e-2_f64) * t20631 + F::cast_from(0.68598428988911579156e-2_f64) * t1173 * t4680 * t6270 - F::cast_from(0.25724410870841842183e-2_f64) * t1180 * t1165 * t4313 * t1759 * t1131 - F::cast_from(0.68598428988911579156e-2_f64) * t15622;
    t20642
}
