//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1775/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1775<F: Float>(t1161: F, t1169: F, t1180: F, t1188: F, t12472: F, t12555: F, t1745: F, t1757: F, t20526: F, t20542: F, t24363: F, t24366: F, t24408: F, t24411: F, t45085: F, t45157: F, t45159: F, t45177: F, t45188: F, t45190: F, t5120: F, t5158: F, t58005: F, t58247: F, t6503: F, t6506: F, t6535: F, t6538: F, t69359: F, t69376: F, t81791: F, t82050: F, t90327: F, t90357: F, t90499: F, t90670: F, t90688: F, t90701: F, t90717: F, t90732: F) -> F {
    let t90745 = F::cast_from(0.23392894490538584828e1_f64) * t5158 * t24408 + F::cast_from(0.4101607543286562663e4_f64) * t58247 * t24411 - F::cast_from(0.12304822629859687989e5_f64) * t45177 * t90357 * t12555 + F::cast_from(0.5848223622634646207e0_f64) * t1180 * t90499 * t1188 + F::cast_from(0.91082604192152556044e5_f64) * t45188 * t90357 * t45190 + F::cast_from(4.0_f64) * t81791 * t1745 - t90327 + F::cast_from(0.23392894490538584828e1_f64) * t82050 * t1757 + F::cast_from(6.0_f64) * t20542 * t6503 + F::cast_from(4.0_f64) * t5120 * t24363 + F::cast_from(0.1929837539843104208e3_f64) * t69376 * t6506 + F::cast_from(0.82761620670837440481e4_f64) * t58005 * t24366 - F::cast_from(0.24828486201251232145e5_f64) * t45085 * t90670 * t12472 + F::cast_from(1.0_f64) * t1161 * (t90688 + t90701 + t90717 + t90732) * t1169 + F::cast_from(0.19964560303604640732e6_f64) * t45157 * t90670 * t45159 + F::cast_from(0.35089341735807877242e1_f64) * t20526 * t6535 + F::cast_from(0.10389515463408878255e3_f64) * t69359 * t6538;
    t90745
}
