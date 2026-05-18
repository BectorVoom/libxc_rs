//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 166/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk166<F: Float>(t24: F, t507: F, t91: F, t506: F, t98: F, zeta_threshold: F) -> F {
    let t90 = t24 <= zeta_threshold;
    let t510 = piecewise3::<f64>(t90, F::new(0.0), F::new(4.0) / F::new(3.0) * t91 * t507);
    let t512 = (t506 + t510) * t98;
    t512
}
