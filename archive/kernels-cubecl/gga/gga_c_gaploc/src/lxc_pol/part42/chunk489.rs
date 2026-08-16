//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 489/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk489<F: Float>(t2312: F, t3122: F, t484: F, t3130: F, t1339: F, t475: F, t123: F, t6514: F, t2326: F, t9074: F, t4261: F, t6510: F) -> (F, F, F, F, F, F) {
    let t9147 = F::cast_from(0.23712505529730124666e-2_f64) * t2312 * t3122;
    let t9149 = F::cast_from(0.31616674039640166221e-2_f64) * t484 * t3122;
    let t9151 = F::cast_from(0.31616674039640166221e-2_f64) * t484 * t3130;
    let t9182 = t1339 * t475;
    let t9204 = t6514 * t123;
    let t9205 = t9204 * t2326;
    let t9207 = F::cast_from(0.71137516589190373998e-2_f64) * t9074 * t9205;
    let t9208 = t4261 * t6510;
    (t9147, t9149, t9151, t9182, t9207, t9208)
}
