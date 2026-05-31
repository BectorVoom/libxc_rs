//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2882/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2882<F: Float>(t11450: F, t1621: F, t11404: F, t11409: F, t11410: F, t11444: F, t11461: F, t11466: F, t11501: F, t15235: F, t15241: F, t15258: F, t15259: F, t15263: F, t15267: F, t15283: F, t15287: F, t15340: F, t1622: F, t1634: F, t2943: F, t2944: F, t2962: F, t2987: F, t2988: F, t3006: F, t41667: F, t41751: F, t41756: F, t41895: F, t4670: F, t4708: F, t4712: F, t953: F, t972: F) -> F {
    let t52320 = t11450 * t1621;
    let t52324 = F::cast_from(0.51947577317044391277e2_f64) * t41751 * t4712 + F::cast_from(0.10389515463408878255e3_f64) * t11461 * t15259 + F::cast_from(0.51947577317044391277e2_f64) * t11461 * t15263 + F::cast_from(0.30762056574649219973e4_f64) * t41756 * t15267 - F::cast_from(0.35089341735807877242e1_f64) * t2987 * t15235 * t972 - F::cast_from(0.35089341735807877242e1_f64) * t2987 * t4708 * t3006 - F::cast_from(0.31168546390226634765e3_f64) * t11466 * t15258 * t2988 - F::cast_from(0.11696447245269292414e1_f64) * t2987 * t1634 * t11501 - F::cast_from(6.0_f64) * t2943 * t15340 * t953 - F::cast_from(6.0_f64) * t2943 * t4670 * t2962 - F::cast_from(0.57895126195293126242e3_f64) * t11409 * t15283 * t2944 - F::cast_from(2.0_f64) * t2943 * t1622 * t11444 - F::cast_from(0.24828486201251232145e5_f64) * t41667 * t15241 * t11410 + F::cast_from(18.0_f64) * t11404 * t15287 + F::cast_from(0.6207121550312808036e4_f64) * t52320 * t41895 * t953;
    t52324
}
