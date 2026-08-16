//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1775/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1775(t1161: f64, t1169: f64, t1180: f64, t1188: f64, t12472: f64, t12555: f64, t1745: f64, t1757: f64, t20526: f64, t20542: f64, t24363: f64, t24366: f64, t24408: f64, t24411: f64, t45085: f64, t45157: f64, t45159: f64, t45177: f64, t45188: f64, t45190: f64, t5120: f64, t5158: f64, t58005: f64, t58247: f64, t6503: f64, t6506: f64, t6535: f64, t6538: f64, t69359: f64, t69376: f64, t81791: f64, t82050: f64, t90327: f64, t90357: f64, t90499: f64, t90670: f64, t90688: f64, t90701: f64, t90717: f64, t90732: f64) -> f64 {
    let t90745 = 0.23392894490538584828e1_f64 * t5158 * t24408 + 0.4101607543286562663e4_f64 * t58247 * t24411 - 0.12304822629859687989e5_f64 * t45177 * t90357 * t12555 + 0.5848223622634646207e0_f64 * t1180 * t90499 * t1188 + 0.91082604192152556044e5_f64 * t45188 * t90357 * t45190 + 4.0_f64 * t81791 * t1745 - t90327 + 0.23392894490538584828e1_f64 * t82050 * t1757 + 6.0_f64 * t20542 * t6503 + 4.0_f64 * t5120 * t24363 + 0.1929837539843104208e3_f64 * t69376 * t6506 + 0.82761620670837440481e4_f64 * t58005 * t24366 - 0.24828486201251232145e5_f64 * t45085 * t90670 * t12472 + 1.0_f64 * t1161 * (t90688 + t90701 + t90717 + t90732) * t1169 + 0.19964560303604640732e6_f64 * t45157 * t90670 * t45159 + 0.35089341735807877242e1_f64 * t20526 * t6535 + 0.10389515463408878255e3_f64 * t69359 * t6538;
    t90745
}
