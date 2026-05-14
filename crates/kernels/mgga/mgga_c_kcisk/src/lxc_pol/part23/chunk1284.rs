//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1284/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1284<F: Float>(t32112: F, t9442: F, t32176: F, t32189: F, t53214: F, t9428: F, t9446: F, t9426: F, t32096: F, t32173: F, t32019: F, t20160: F, t32179: F, t32042: F, t32066: F, t32084: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t110683 = t32112 * t9442;
    let t110687 = t32189 * t32176;
    let t110691 = t53214 * t9428;
    let t110692 = t9446 * t110691;
    let t110695 = t9426 * t110691;
    let t110697 = t32096 * t32173;
    let t110699 = t32019 * t32173;
    let t110702 = t9446 * t20160 * t32179;
    let t110704 = t32096 * t32176;
    let t110725 = t32019 * t32042;
    let t110748 = t32066 * t32176;
    let t110754 = t32084 * t9442;
    (t110683, t110687, t110692, t110695, t110697, t110699, t110702, t110704, t110725, t110748, t110754)
}
