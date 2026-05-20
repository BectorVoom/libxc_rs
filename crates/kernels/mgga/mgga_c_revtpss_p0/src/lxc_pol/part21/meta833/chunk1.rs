//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3116/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3116<F: Float>(t1263: F, t16750: F, t1012: F, t1042: F, t1122: F, t1222: F, t1225: F, t12787: F, t12836: F, t12956: F, t13002: F, t13008: F, t17502: F, t17605: F, t17736: F, t17737: F, t3625: F, t3626: F, t3629: F, t3711: F, t49889: F, t5046: F, t5373: F, t57083: F, t57257: F, t57258: F, t57265: F, t57271: F, t57274: F, t57275: F, t57290: F, t57292: F, t57295: F, t57297: F, t57299: F) -> F {
    let t57303 = t1263 * t16750;
    let t57308 = t57257 + F::cast_from(0.47637797908966374413e-3_f64) * t57258 + F::cast_from(0.14291339372689912324e-2_f64) * t17736 * t12787 * t5046 * t57083 + F::cast_from(0.25724410870841842184e-2_f64) * t57265 * t3626 * t17737 * t57083 - t57271 + t57274 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t3626 * t57275 * t3629 + F::cast_from(0.22866142996303859718e-2_f64) * t17605 * t12836 - t1222 * t1012 * t1225 * t49889 / F::new(288.0) + t5373 * t13002 / F::new(108.0) + t5373 * t13008 / F::new(18.0) + t57290 / F::new(108.0) + t57292 / F::new(54.0) - t57295 / F::new(288.0) - F::cast_from(0.22866142996303859718e-2_f64) * t57297 + F::cast_from(0.42874018118069736972e-3_f64) * t57299 + F::cast_from(0.85748036236139473944e-3_f64) * t12956 * t17502 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t1042 * t57303 * t1122;
    t57308
}
