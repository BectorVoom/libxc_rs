//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 520/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk520<F: Float>(t674: F, t7407: F, t1993: F, t2185: F, t1997: F, t4179: F, t6: F, t220: F, t211: F, t483: F, t1965: F) -> (F, F, F, F, F, F, F) {
    let t7408 = t7407 * t674;
    let t7414 = t1993 * t2185;
    let t7415 = t7414 * t1997;
    let t7416 = 0.24829349937757072982e-4 * t7415;
    let t7417 = t6 * t4179;
    let t7418 = t220 * t7417;
    let t7427 = t211 * t483;
    let t7428 = t1965 * t7427;
    (t7408, t7414, t7416, t7417, t7418, t7427, t7428)
}
