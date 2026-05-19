//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1107/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1107<F: Float>(t2679: F, t47168: F, t9805: F, t1991: F, t47130: F, t590: F, t739: F, t43413: F, t43414: F, t43417: F, t43421: F, t43426: F, t47155: F, t47157: F, t47160: F, t47164: F, t47166: F) -> F {
    let t47170 = t9805 * t47168 * t2679;
    let t47174 = t1991 * t739 * t47130 * t590;
    let t47176 = -t43413 + t43414 - t43417 + t47155 - F::cast_from(0.57514388930881124514e0_f64) * t43421 + t47157 + t47160 - t47164 + F::cast_from(0.9585731488480187419e0_f64) * t47166 - F::cast_from(0.57514388930881124514e0_f64) * t47170 + F::cast_from(0.1022478025437886658e1_f64) * t47174 - t43426;
    t47176
}
