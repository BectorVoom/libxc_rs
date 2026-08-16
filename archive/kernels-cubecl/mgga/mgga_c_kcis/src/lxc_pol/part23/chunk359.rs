//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 359/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk359<F: Float>(t1629: F, t187: F, t2017: F, t2070: F, t2118: F, t2128: F, t633: F, t69: F, t706: F, t74: F) -> (F, F) {
    let t2132 = t2017 - t2070 + t187 * (-t1629 * t2128 + t2118 * t633 - t2017 + t2070);
    let t2140 = t69 * t74 * t706;
    (t2132, t2140)
}
