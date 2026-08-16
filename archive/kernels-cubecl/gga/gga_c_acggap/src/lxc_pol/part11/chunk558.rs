//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 558/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk558<F: Float>(t322: F, t3828: F, t381: F, t1240: F, t879: F, t1004: F, t1241: F, t1248: F, t377: F, t1261: F, t310: F, t1244: F) -> (F, F, F, F, F, F) {
    let t3829 = t3828 * t322;
    let t3830 = t381 * t3829;
    let t3832 = t1240 * t879;
    let t3833 = t381 * t3832;
    let t3835 = t1004 * t1241;
    let t3837 = t377 * t1248;
    let t3839 = t310 * t1261;
    let t3842 = F::cast_from(0.19756347548806534796e1_f64) * t1004 * t1244;
    (t3830, t3833, t3835, t3837, t3839, t3842)
}
