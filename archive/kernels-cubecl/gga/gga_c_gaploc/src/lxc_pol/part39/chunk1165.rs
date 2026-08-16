//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1165/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1165<F: Float>(t1843: F, t39149: F, t7064: F, t12255: F, t2508: F, t2586: F, t43244: F, t43248: F, t43254: F, t43257: F, t43260: F, t43263: F, t43265: F, t43267: F, t43269: F) -> F {
    let t47731 = t7064 * t1843 * t39149;
    let t47734 = t2508 * t12255 * t2586;
    let t47736 = -F::cast_from(0.23071578690426672851e-1_f64) * t43244 - F::cast_from(0.23071578690426672851e-1_f64) * t43248 + t43254 + t43257 + F::cast_from(0.32043859292259267849e-3_f64) * t47731 + t43260 + t43263 + t43265 - t43267 - t43269 - F::cast_from(0.23071578690426672851e-1_f64) * t47734;
    t47736
}
