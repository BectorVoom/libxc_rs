//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2764/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2764<F: Float>(t22025: F, t2661: F, t5675: F, t9934: F, t6836: F, t9940: F, t1353: F, t13767: F, t13768: F, t5591: F, t21969: F, t221: F, t3978: F, t3979: F) -> (F, F, F, F) {
    let t73985 = t2661 * t9934 * t22025 * t5675;
    let t73991 = t9940 * t6836;
    let t73994 = t2661 * t13767 * t73991 * t1353;
    let t73998 = t2661 * t13767 * t13768 * t5591;
    let t74010 = t3978 * t3979 * t221 * t21969;
    (t73985, t73994, t73998, t74010)
}
