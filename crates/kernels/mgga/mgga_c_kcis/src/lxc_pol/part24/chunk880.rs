//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 880/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk880<F: Float>(t18866: F, t18868: F, t18870: F, t18872: F, t18874: F, t18947: F, t18949: F, t18970: F, t18973: F, t18976: F, t18980: F, t20377: F, t405: F, t6400: F, t962: F, t19094: F, t971: F) -> (F, F, F) {
    let t20380 = -0.3109e-1 * t20377 * t405 - t18866 - t18868 - t18870 + t18872 - t18874 - t18947 - t18949 + t18970 + t18973 + t18976 - t18980;
    let t20381 = t6400 * t962;
    let t20392 = t19094 * t971;
    (t20380, t20381, t20392)
}
