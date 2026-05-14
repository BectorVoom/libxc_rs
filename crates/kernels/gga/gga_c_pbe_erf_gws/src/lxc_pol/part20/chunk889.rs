//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 889/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk889<F: Float>(t643: F, t9801: F, t642: F, t639: F, t1627: F, t3523: F, t1791: F, t3390: F, t617: F, t1621: F, t1620: F, t5109: F, t661: F, t2615: F, t2689: F, t2556: F) -> (F, F, F, F, F, F) {
    let t11125 = t643 * t9801;
    let t11126 = t642 * t11125;
    let t11128 = 4.0 / 45.0 * t639 * t11126;
    let t11130 = 4.0 / 27.0 * t1627 * t3523;
    let t11131 = t1791 * t3390;
    let t11132 = t11131 * t617;
    let t11133 = t1621 * t11132;
    let t11135 = 8.0 / 15.0 * t1620 * t11133;
    let t11136 = t5109 * t3390;
    let t11137 = t11136 * t661;
    let t11138 = t1621 * t11137;
    let t11140 = 4.0 / 5.0 * t639 * t11138;
    let t11142 = 8.0 / 45.0 * t2615 * t2689;
    let t11144 = 16.0 / 45.0 * t2615 * t2556;
    (t11128, t11130, t11135, t11140, t11142, t11144)
}
