//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 857/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk857<F: Float>(t1210: F, t618: F, t2137: F, t441: F, t7923: F, t615: F, t2130: F, t7922: F, t861: F, t14651: F, t159: F, t448: F, t7911: F) -> (F, F, F, F, F, F, F, F) {
    let t29984 = t1210 * t618;
    let t29985 = t2137 * t29984;
    let t29991 = t7923 * t441;
    let t29994 = t615 * t29984;
    let t30005 = t7923 * t2130;
    let t30009 = t7922 * t861 * t2130;
    let t30023 = t14651 * t159;
    let t30028 = t7911 * t448;
    (t29984, t29985, t29991, t29994, t30005, t30009, t30023, t30028)
}
