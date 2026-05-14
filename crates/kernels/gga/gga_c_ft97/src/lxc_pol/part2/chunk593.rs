//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 593/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk593<F: Float>(t40: F, t6: F, t12: F, t171: F, t341: F, t630: F, t343: F, t70: F, t120: F, t358: F, t363: F, t123: F, t532: F, t7911: F, t122: F, t29: F, t32: F) -> (F, F, F, F, F, F, F) {
    let t8946 = t6 / t40;
    let t8947 = t12 * t171;
    let t8948 = t8946 * t8947;
    let t8959 = t341 * t630;
    let t8963 = t341 * t343 * t70;
    let t8965 = t120 * t358;
    let t8966 = t8965 * t363;
    let t8977 = t123 / t532 / t7911;
    let t8991 = t122 * t122;
    let t8994 = t8991 / t32 / t29;
    (t8948, t8959, t8963, t8966, t8977, t8991, t8994)
}
