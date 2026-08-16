//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2882/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2882(t11450: f64, t1621: f64, t11404: f64, t11409: f64, t11410: f64, t11444: f64, t11461: f64, t11466: f64, t11501: f64, t15235: f64, t15241: f64, t15258: f64, t15259: f64, t15263: f64, t15267: f64, t15283: f64, t15287: f64, t15340: f64, t1622: f64, t1634: f64, t2943: f64, t2944: f64, t2962: f64, t2987: f64, t2988: f64, t3006: f64, t41667: f64, t41751: f64, t41756: f64, t41895: f64, t4670: f64, t4708: f64, t4712: f64, t953: f64, t972: f64) -> f64 {
    let t52320 = t11450 * t1621;
    let t52324 = 0.51947577317044391277e2_f64 * t41751 * t4712 + 0.10389515463408878255e3_f64 * t11461 * t15259 + 0.51947577317044391277e2_f64 * t11461 * t15263 + 0.30762056574649219973e4_f64 * t41756 * t15267 - 0.35089341735807877242e1_f64 * t2987 * t15235 * t972 - 0.35089341735807877242e1_f64 * t2987 * t4708 * t3006 - 0.31168546390226634765e3_f64 * t11466 * t15258 * t2988 - 0.11696447245269292414e1_f64 * t2987 * t1634 * t11501 - 6.0_f64 * t2943 * t15340 * t953 - 6.0_f64 * t2943 * t4670 * t2962 - 0.57895126195293126242e3_f64 * t11409 * t15283 * t2944 - 2.0_f64 * t2943 * t1622 * t11444 - 0.24828486201251232145e5_f64 * t41667 * t15241 * t11410 + 18.0_f64 * t11404 * t15287 + 0.6207121550312808036e4_f64 * t52320 * t41895 * t953;
    t52324
}
