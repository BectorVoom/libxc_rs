//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 779/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk779<F: Float>(t29979: F, t29980: F, t620: F, t1210: F, t618: F, t2137: F, t2140: F, t2122: F, t310: F, t464: F, t441: F, t7923: F, t621: F, t615: F, t394: F) -> (F, F, F, F, F, F, F, F) {
    let t29982 = t29979 * t620 * t29980;
    let t29984 = t1210 * t618;
    let t29985 = t2137 * t29984;
    let t29986 = t29985 * t2140;
    let t29988 = t310 * t2122;
    let t29989 = t29988 * t464;
    let t29991 = t7923 * t441;
    let t29992 = t29991 * t621;
    let t29994 = t615 * t29984;
    let t29997 = t394 * t2122;
    (t29982, t29984, t29986, t29988, t29989, t29992, t29994, t29997)
}
