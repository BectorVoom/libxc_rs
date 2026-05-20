//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1922/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1922<F: Float>(t3154: F, t4866: F, t4893: F, t3117: F, t11922: F, t6272: F, t3115: F, t1668: F, t3181: F, t372: F, t1045: F, t4574: F) -> (F, F, F, F, F, F, F, F) {
    let t19971 = t3154 * t4866;
    let t19972 = t4893 * t19971;
    let t19973 = t3117 * t19972;
    let t19976 = t11922 * t6272;
    let t19977 = t3115 * t19976;
    let t19979 = t3181 * t1668;
    let t19980 = t372 * t19979;
    let t19981 = t1045 * t4574;
    (t19971, t19972, t19973, t19976, t19977, t19979, t19980, t19981)
}
