//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1401/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1401(t12000: f64, t12116: f64, t1339: f64, t1537: f64, t1646: f64, t31190: f64, t31213: f64, t31215: f64, t31216: f64, t34954: f64, t34957: f64, t34959: f64, t34962: f64, t34964: f64, t34967: f64, t34970: f64, t34973: f64, t34976: f64, t34979: f64, t528: f64, t590: f64) -> f64 {
    let t38811 = -0.71500979903700853338e0_f64 * t528 * t12116 * t1646 - t31190 - t34954 - t31213 - t31215 + 0.20449560508757733162e1_f64 * t31216 - 0.1022478025437886658e1_f64 * t1537 * t1339 * t12000 * t590 + t34957 - t34959 + t34962 - t34964 - t34967 - t34970 + t34973 - t34976 + t34979;
    t38811
}
