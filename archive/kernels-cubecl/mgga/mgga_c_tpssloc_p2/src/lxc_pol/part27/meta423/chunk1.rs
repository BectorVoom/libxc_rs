//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1733/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1733<F: Float>(t2006: F, t3752: F, t1323: F, t6955: F, t2015: F, t3888: F, t12021: F, t1887: F, t6916: F) -> (F, F, F, F) {
    let t22622 = t3752 * t2006;
    let t22624 = t1323 * t6955;
    let t22629 = t2015 * t3888;
    let t22630 = t12021 * t22629;
    let t22633 = t6916 * t1887;
    (t22622, t22624, t22630, t22633)
}
