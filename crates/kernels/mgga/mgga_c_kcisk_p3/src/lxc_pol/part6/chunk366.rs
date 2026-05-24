//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 366/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk366<F: Float>(t2260: F, t2264: F, t2268: F, t2272: F, t2276: F, t2280: F) -> F {
    let t2347 = F::new(0.9375e-1) * t2260 - F::new(0.9375e-1) * t2264 + F::new(0.625e-1) * t2268 - F::cast_from(0.101171875e-1_f64) * t2272 + F::cast_from(0.101171875e-1_f64) * t2276 - F::cast_from(0.13489583333333333333e-1_f64) * t2280;
    t2347
}
