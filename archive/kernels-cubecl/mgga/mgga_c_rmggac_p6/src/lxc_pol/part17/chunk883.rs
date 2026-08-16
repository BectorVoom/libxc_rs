//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 883/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk883<F: Float>(t2010: F, t2415: F, t4962: F, t5002: F, t7487: F, t9723: F, t2011: F, t291: F, t5878: F, t935: F, t9719: F, t938: F) -> (F, F, F, F, F, F) {
    let t44825 = t2010 * t2415 * t4962;
    let t44828 = t2010 * t2415 * t5002;
    let t44831 = t7487 * t9723;
    let t44835 = t2010 * t2011 * t5878 * t291;
    let t44838 = t2010 * t9719 * t935;
    let t44841 = t2010 * t9719 * t938;
    (t44825, t44828, t44831, t44835, t44838, t44841)
}
