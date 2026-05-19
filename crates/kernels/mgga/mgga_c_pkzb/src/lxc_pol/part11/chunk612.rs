//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 612/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk612<F: Float>(t24: F, t1651: F, t3371: F, t3374: F, t91: F, t3370: F, t98: F, zeta_threshold: F) -> F {
    let t90 = t24 <= zeta_threshold;
    let t3378 = piecewise3::<F>(t90, F::new(0.0), F::new(4.0) / F::new(9.0) * t1651 * t3371 + F::new(4.0) / F::new(3.0) * t91 * t3374);
    let t3380 = (t3370 + t3378) * t98;
    t3380
}
