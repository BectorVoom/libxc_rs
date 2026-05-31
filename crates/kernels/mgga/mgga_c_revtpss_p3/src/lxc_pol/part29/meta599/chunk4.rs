//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2043/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2043<F: Float>(t10416: F, t13426: F, t13435: F, t18153: F, t18227: F, t1843: F, t2014: F, t2052: F, t2320: F, t2322: F, t25089: F, t25188: F, t26153: F, t26376: F, t26380: F, t26396: F, t26406: F, t27833: F, t28196: F, t28286: F, t28586: F, t28704: F, t28709: F, t28938: F, t4248: F, t5542: F, t649: F, t651: F, t7235: F, t7374: F, t7489: F, t7539: F, t7898: F, t7984: F, t8065: F, t8109: F, t98450: F, t98550: F) -> F {
    let t104038 = t25188 * t8109 - F::cast_from(4.0_f64) * t13426 * t7374 - F::cast_from(4.0_f64) * t18227 * t7374 - F::cast_from(4.0_f64) * t4248 * t26396 - F::cast_from(2.0_f64) * t7235 * t28709 + F::cast_from(3.0_f64) * t2014 * t28938 * t25089 - F::cast_from(2.0_f64) * t7898 * t26380 - F::cast_from(6.0_f64) * t98450 * t26406 - t2052 * t18153 + F::cast_from(2.0_f64) * t28196 * t28286 * t98550 + F::cast_from(6.0_f64) * t27833 * t7489 - t2014 * t26376 * t5542 - t2320 * t8065 - F::cast_from(2.0_f64) * t649 * t28586 - F::cast_from(2.0_f64) * t27833 * t7539 - F::cast_from(2.0_f64) * t651 * t1843 * t26153 - F::cast_from(2.0_f64) * t10416 * t7984 - F::cast_from(4.0_f64) * t13435 * t7984 - F::cast_from(4.0_f64) * t2322 * t28704;
    t104038
}
