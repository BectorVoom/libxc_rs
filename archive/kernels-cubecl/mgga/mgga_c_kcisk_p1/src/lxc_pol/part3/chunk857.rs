//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 857/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk857<F: Float>(t12916: F, t12919: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12935: F, t12937: F, t12939: F, t12943: F, t12946: F, t12948: F, t12954: F) -> F {
    let t12956 = -F::cast_from(0.28483875e1_f64) * t12916 + F::cast_from(0.46074375e0_f64) * t12919 - F::cast_from(0.33218518518518518518e0_f64) * t12922 - F::cast_from(0.29896666666666666667e0_f64) * t12927 - F::cast_from(0.39862222222222222223e0_f64) * t12929 + F::cast_from(0.29896666666666666667e0_f64) * t12931 + F::cast_from(0.19931111111111111111e0_f64) * t12933 - F::cast_from(0.27385555555555555556e0_f64) * t12935 + F::cast_from(0.16431333333333333333e0_f64) * t12937 + F::cast_from(0.5477111111111111111e-1_f64) * t12939 - F::cast_from(0.36514074074074074075e-1_f64) * t12943 - F::cast_from(0.82156666666666666667e-1_f64) * t12946 - F::cast_from(0.59793333333333333333e0_f64) * t12948 + F::cast_from(0.11958666666666666667e1_f64) * t12954;
    t12956
}
