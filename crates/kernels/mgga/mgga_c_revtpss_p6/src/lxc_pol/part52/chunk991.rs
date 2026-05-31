//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 991/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk991<F: Float>(t1312: F, t13426: F, t1518: F, t18227: F, t2055: F, t2322: F, t26399: F, t27123: F, t28219: F, t28652: F, t28653: F, t28658: F, t28683: F, t4248: F, t4292: F, t5523: F, t670: F, t7359: F, t7373: F, t7889: F, t7983: F) -> F {
    let t28686 = F::cast_from(2.0_f64) * t1312 * t28683 + F::cast_from(2.0_f64) * t13426 * t2055 + F::cast_from(2.0_f64) * t1518 * t26399 + F::cast_from(2.0_f64) * t1518 * t28658 + F::cast_from(2.0_f64) * t18227 * t2055 + F::cast_from(2.0_f64) * t2055 * t27123 + F::cast_from(2.0_f64) * t2055 * t28219 + F::cast_from(2.0_f64) * t2322 * t7983 + F::cast_from(2.0_f64) * t28653 * t670 + F::cast_from(2.0_f64) * t4248 * t7373 + F::cast_from(2.0_f64) * t4292 * t7359 + F::cast_from(2.0_f64) * t5523 * t7983 + F::cast_from(2.0_f64) * t7373 * t7889 + t28652;
    t28686
}
