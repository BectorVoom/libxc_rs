//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 772/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk772<F: Float>(t1587: F, t2084: F, t2134: F, t27: F, t7512: F, t8368: F, t36471: F, t5145: F, t656: F, t34938: F, t5149: F, t1550: F, t2060: F, t27059: F, t2347: F, t876: F) -> (F, F, F, F, F, F) {
    let t39031 = t2134 * t27 * t2084 * t1587;
    let t39033 = t8368 * t7512;
    let t39036 = t36471 * t656 * t5145;
    let t39039 = t34938 * t656 * t5149;
    let t39042 = t1550 * t2060 * t27059;
    let t39044 = t2347 * t876;
    (t39031, t39033, t39036, t39039, t39042, t39044)
}
