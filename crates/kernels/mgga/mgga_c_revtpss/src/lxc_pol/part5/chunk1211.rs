//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1211/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1211<F: Float>(t21082: F, t482: F, t371: F, t372: F, t5323: F, t5362: F, t12772: F, t6639: F, t3625: F, t1263: F, t6573: F, t1122: F, t1042: F, t1038: F, t6593: F, t1244: F) -> (F, F, F, F, F) {
    let t21083 = t482 * t21082;
    let t21085 = t371 * t372 * t21083;
    let t21088 = t5323 * t5362;
    let t21090 = t12772 * t6639;
    let t21091 = t3625 * t21090;
    let t21093 = t1263 * t6573;
    let t21094 = t21093 * t1122;
    let t21095 = t1042 * t21094;
    let t21100 = t6593 * t1038;
    let t21101 = t1244 * t21100;
    (t21085, t21088, t21091, t21095, t21101)
}
