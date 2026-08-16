//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 662/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk662<F: Float>(t2264: F, t2269: F, t3017: F, t3059: F, t3732: F, t3744: F, t3748: F, t3752: F, t3754: F, t3759: F, t3763: F) -> F {
    let t3792 = -F::cast_from(0.17648625e1_f64) * t3744 + F::cast_from(0.3529725e1_f64) * t3748 + t2264 - F::cast_from(0.103295e1_f64) * t3017 + F::cast_from(0.1549425e1_f64) * t3732 + F::cast_from(0.31558125e0_f64) * t3752 + F::cast_from(0.6311625e0_f64) * t3754 + t2269 - F::cast_from(0.41678e0_f64) * t3059 + F::cast_from(0.312585e0_f64) * t3759 + F::cast_from(0.312585e0_f64) * t3763;
    t3792
}
