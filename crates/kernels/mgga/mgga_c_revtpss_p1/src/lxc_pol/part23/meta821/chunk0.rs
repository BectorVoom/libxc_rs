//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2671/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2671<F: Float>(t11999: F, t19826: F, t11262: F, t3150: F, t6307: F, t11710: F, t19725: F, t4892: F, t15669: F, t16088: F, t380: F, t1045: F, t4186: F) -> (F, F, F, F, F) {
    let t66024 = t11999 * t19826;
    let t66029 = t3150 * t11262 * t6307;
    let t66043 = t4892 * t11710 * t19725;
    let t66047 = t15669 * t380 * t16088;
    let t66066 = t1045 * t4186;
    (t66024, t66029, t66043, t66047, t66066)
}
