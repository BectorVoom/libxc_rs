//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 626/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk626<F: Float>(t1652: F, t36: F, t2079: F, t262: F, t2024: F, t570: F, t664: F) -> (F, F, F, F) {
    let t8924 = t36 * t1652;
    let t8926 = t2079 * t262 * t8924;
    let t8933 = t2024 * t1652;
    let t8936 = t664 * t570;
    (t8924, t8926, t8933, t8936)
}
