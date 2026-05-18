//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 830/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk830<F: Float>(t1713: F, t599: F, t142: F, t7450: F, t2313: F, t507: F, t2030: F, t2317: F, t2060: F, t1849: F, t604: F, t1181: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9659 = t599 * t1713;
    let t9660 = t142 * t9659;
    let t9661 = t7450 * t9660;
    let t9663 = t507 * t2313;
    let t9664 = t2030 * t9663;
    let t9666 = t507 * t2317;
    let t9667 = t2060 * t9666;
    let t9669 = t604 * t1849;
    let t9670 = t1181 * t9669;
    (t9659, t9660, t9661, t9663, t9664, t9666, t9667, t9669, t9670)
}
