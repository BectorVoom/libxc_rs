//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 904/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk904<F: Float>(t24: F, t1003: F, t5106: F, t1651: F, t8: F, t1429: F, t507: F, t1652: F, t1655: F, t2548: F, t2551: F, t82: F, t91: F, zeta_threshold: F) -> (F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t6782 = t5106 * t1003;
    let t6785 = t1651 * t8;
    let t6786 = t1429 * t507;
    let t6796 = piecewise3::<F>(t90, F::new(0.0), -F::new(8.0) / F::new(27.0) * t6782 * t1652 - F::new(16.0) / F::new(9.0) * t6785 * t6786 + F::new(4.0) / F::new(9.0) * t2548 * t1655 - F::new(8.0) / F::new(3.0) * t91 * t1429 + F::new(8.0) * t2551 * t82);
    (t6782, t6785, t6786, t6796)
}
