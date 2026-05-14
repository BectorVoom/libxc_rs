//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1006/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1006<F: Float>(t20684: F, t2192: F, t2197: F, t27053: F, t27947: F, t27969: F, t28235: F, t28945: F, t28962: F, t28967: F, t28974: F, t28993: F, t29001: F, t29004: F, t29120: F, t29143: F, t29170: F) -> (F, F) {
    let t29172 = t20684 * t2192;
    let t29184 = 0.11607361111111111111e-2 * t28945 - t27053 - 0.34752604166666666667e-3 * t29172 * t2197 - 0.34822083333333333332e-2 * t28962 + 0.23214722222222222222e-2 * t28967 + 0.23168402777777777778e-3 * t28235 - 0.17411041666666666666e-2 * t28974 + 0.23214722222222222222e-2 * t27947 - 0.23214722222222222222e-2 * t28993 - 0.38691203703703703703e-3 * t29001 + 0.34822083333333333332e-2 * t29004 - 0.23214722222222222222e-2 * t27969;
    let t29186 = t29120 + t29143 + t29170 + t29184;
    (t29172, t29186)
}
