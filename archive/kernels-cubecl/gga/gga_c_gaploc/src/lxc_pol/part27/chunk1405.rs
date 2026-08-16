//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1405/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1405<F: Float>(t12000: F, t12116: F, t1339: F, t1537: F, t1646: F, t31190: F, t31213: F, t31215: F, t31216: F, t34954: F, t34957: F, t34959: F, t34962: F, t34964: F, t34967: F, t34970: F, t34973: F, t34976: F, t34979: F, t528: F, t590: F) -> F {
    let t38811 = -F::cast_from(0.71500979903700853338e0_f64) * t528 * t12116 * t1646 - t31190 - t34954 - t31213 - t31215 + F::cast_from(0.20449560508757733162e1_f64) * t31216 - F::cast_from(0.1022478025437886658e1_f64) * t1537 * t1339 * t12000 * t590 + t34957 - t34959 + t34962 - t34964 - t34967 - t34970 + t34973 - t34976 + t34979;
    t38811
}
