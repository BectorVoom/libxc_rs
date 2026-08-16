//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1340/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1340<F: Float>(t21116: F, t62348: F, t6077: F, t67329: F, t21123: F, t5791: F, t18670: F, t21129: F, t21133: F, t19411: F, t19414: F, t19417: F, t20282: F, t21139: F, t21756: F, t5492: F, t5794: F, t6080: F, t62277: F, t6304: F) -> F {
    let t71473 = t62348 * t21116;
    let t71475 = t67329 * t6077;
    let t71477 = t21123 * t5791;
    let t71479 = t18670 * t21129;
    let t71481 = t18670 * t21133;
    let t71487 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t21139 * t5794 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t19411 * t6304 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t19414 * t6304 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t19417 * t6304 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t6080 * t20282 - F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t71473 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t71475 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t71477 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t71479 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t71481 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5492 * t21756 + F::cast_from(10.0_f64) * t62277 * t21116;
    t71487
}
