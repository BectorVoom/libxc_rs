//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 595/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk595<F: Float>(t3896: F, t464: F, t852: F, t880: F, t441: F, t851: F, t323: F, t1222: F, t857: F, t872: F, t1221: F, t322: F, t1220: F, t316: F, t3101: F, t317: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3897 = t3896 * t464;
    let t3900 = 0.19756347548806534796e1 * t852 * t880;
    let t3901 = t851 * t441;
    let t3902 = t3901 * t323;
    let t3904 = t857 * t1222;
    let t3906 = t852 * t872;
    let t3908 = t322 * t1221;
    let t3909 = t1220 * t3908;
    let t3910 = t316 * t3909;
    let t3912 = t317 * t3101;
    (t3897, t3900, t3901, t3902, t3904, t3906, t3908, t3909, t3910, t3912)
}
