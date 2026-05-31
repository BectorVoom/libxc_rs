//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1464/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1464<F: Float>(t22465: F, t22473: F, t22482: F, t22504: F, t1312: F, t13426: F, t1518: F, t18220: F, t18227: F, t18245: F, t21814: F, t21881: F, t2322: F, t4248: F, t4292: F, t5523: F, t5920: F, t670: F, t7889: F) -> (F, F) {
    let t22506 = t22465 + t22473 + t22482 + t22504;
    let t22525 = F::cast_from(2.0_f64) * t1312 * t21881 + F::cast_from(4.0_f64) * t13426 * t1518 + F::cast_from(4.0_f64) * t1518 * t18227 + F::cast_from(2.0_f64) * t18245 * t670 + F::cast_from(2.0_f64) * t2322 * t5920 + F::cast_from(4.0_f64) * t4248 * t4292 + F::cast_from(4.0_f64) * t4292 * t7889 + F::cast_from(2.0_f64) * t5523 * t5920 + F::cast_from(2.0_f64) * t18220 + t21814;
    (t22506, t22525)
}
