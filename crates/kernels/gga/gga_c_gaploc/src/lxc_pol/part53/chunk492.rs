//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 492/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk492<F: Float>(t123: F, t9078: F, t4385: F, t1365: F, t6520: F, t6525: F, t6417: F, t883: F, t2325: F, t882: F, t2321: F, t2440: F) -> (F, F, F, F, F, F) {
    let t9079 = t9078 * t123;
    let t9080 = t9079 * t4385;
    let t9083 = t1365 * t6520;
    let t9085 = F::cast_from(0.23712505529730124666e-2_f64) * t6525 * t9083;
    let t9086 = t883 * t6417;
    let t9087 = t2325 * t9086;
    let t9089 = F::cast_from(0.23712505529730124666e-2_f64) * t882 * t9087;
    let t9090 = t2440 * t2321;
    (t9079, t9080, t9085, t9086, t9089, t9090)
}
