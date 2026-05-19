//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 602/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk602<F: Float>(t1365: F, t31: F, t4: F, t1230: F, t1253: F, t1259: F, t1304: F, t1320: F, t440: F, t442: F, t450: F, t4503: F, t4506: F, t4513: F, t4539: F, t4542: F, t4606: F, t4608: F, t4620: F, t4624: F, t4631: F, t4636: F, t4637: F, t4640: F, t4643: F, t71: F, t84: F) -> (F, F, F) {
    let t4651 = t4 * t1365 * t31;
    let t4652 = F::cast_from(0.34451131037037037036e-2_f64) * t4651;
    let t4656 = -t4503 + t4506 + t4513 - t4539 - t4542 - F::cast_from(0.1038945353962551798e3_f64) * t4606 * t4608 + F::cast_from(0.58482233974552040708e0_f64) * t450 * t4620 + F::cast_from(0.51947267698127589897e2_f64) * t1320 * t4624 - F::new(6.0) * t1230 * t442 * t1253 + F::new(6.0) * t1259 * t4631 - F::cast_from(0.19298809906722418785e3_f64) * t4636 * t4637 - F::cast_from(0.35089340384731224426e1_f64) * t1304 * t4640 + F::cast_from(0.96494049533612093922e2_f64) * t1259 * t4643 * t440 + F::cast_from(0.56969282336565386482e-3_f64) * t4 * t1365 * t84 - t4652 + F::cast_from(0.16562449037037037036e-2_f64) * t4 * t1365 * t71;
    (t4651, t4652, t4656)
}
