//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1183/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1183<F: Float>(t24: F, t11146: F, t11150: F, t1165: F, t1430: F, t28895: F, t28898: F, t28906: F, t3019: F, t3022: F, t333: F, t3725: F, t507: F, t8742: F, zeta_threshold: F) -> F {
    let t90 = t24 <= zeta_threshold;
    let t29065 = piecewise3::<F>(t90, F::new(0.0), -F::new(56.0) / F::new(81.0) * t11146 * t507 - F::new(16.0) / F::new(9.0) * t3725 * t1430 + F::new(8.0) / F::new(9.0) * t3019 * t28895 + F::new(4.0) / F::new(3.0) * t3022 * t28898 - F::new(2.0) / F::new(3.0) * t1165 * t8742 - F::new(2.0) / F::new(9.0) * t11150 * t507 + F::new(2.0) / F::new(3.0) * t333 * t28906);
    t29065
}
