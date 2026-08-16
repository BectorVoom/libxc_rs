//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2847/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2847<F: Float>(t141: F, t41294: F, t51856: F, t51865: F, t930: F, t51869: F, t51861: F, t11150: F, t2251: F, t4186: F, t2908: F, t10356: F, t1469: F, t41270: F) -> (F, F, F, F, F, F, F) {
    let t51981 = t141 * t41294 * t51856;
    let t51984 = t141 * t930 * t51865;
    let t51987 = t141 * t930 * t51869;
    let t51990 = t141 * t930 * t51861;
    let t51993 = t11150 * t4186 * t2251;
    let t51995 = t141 * t2908 * t51993;
    let t51998 = t41270 * t1469 * t10356;
    (t51981, t51984, t51987, t51990, t51993, t51995, t51998)
}
