//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1049/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1049<F: Float>(t12461: F, t8803: F, t102344: F, t117014: F, t121004: F, t121007: F, t123368: F, t124293: F, t1458: F, t2039: F, t23938: F, t26977: F, t27170: F, t27188: F, t32235: F, t33234: F, t35233: F, t4072: F, t671: F, t7042: F, t7056: F, t7801: F, t92090: F) -> (F, F) {
    let t124476 = t8803 * t12461;
    let t124531 = F::cast_from(4.0_f64) * t102344 * t2039 + F::cast_from(2.0_f64) * t117014 * t1458 + F::cast_from(4.0_f64) * t121004 * t2039 + F::cast_from(4.0_f64) * t121007 * t2039 + F::cast_from(2.0_f64) * t123368 * t671 + F::cast_from(2.0_f64) * t124293 * t1458 + F::cast_from(4.0_f64) * t2039 * t92090 + F::cast_from(4.0_f64) * t23938 * t7801 + F::cast_from(4.0_f64) * t26977 * t7801 + F::cast_from(4.0_f64) * t27170 * t7042 + F::cast_from(4.0_f64) * t27188 * t7056 + F::cast_from(2.0_f64) * t32235 * t4072 + F::cast_from(4.0_f64) * t33234 * t7056 + F::cast_from(4.0_f64) * t35233 * t7056;
    (t124476, t124531)
}
