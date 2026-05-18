//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1056/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1056<F: Float>(t35554: F, t8571: F, t1970: F, t1971: F, t209: F, t40427: F, t515: F, t275: F, t9031: F, t40884: F, t7204: F, t118: F, t2281: F, t498: F, t7418: F) -> (F, F, F, F, F) {
    let t41897 = t8571 * t35554;
    let t41902 = t1970 * t1971 * t515 * t40427 * t209;
    let t41905 = F::new(2.0) * t275 * t9031;
    let t41906 = t7204 * t40884;
    let t41914 = t7418 * t118 * t2281 * t498;
    (t41897, t41902, t41905, t41906, t41914)
}
