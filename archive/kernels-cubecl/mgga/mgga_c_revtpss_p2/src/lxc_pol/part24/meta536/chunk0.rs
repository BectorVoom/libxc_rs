//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1578/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1578<F: Float>(t22857: F, t550: F, t2661: F, t46609: F, t9994: F, t4003: F, t9934: F, t221: F, t22809: F, t3978: F, t3979: F, t22815: F, t3989: F) -> (F, F, F, F, F) {
    let t86205 = t550 * t22857;
    let t86208 = t2661 * t46609 * t86205 * t9994;
    let t86212 = t2661 * t9934 * t86205 * t4003;
    let t86220 = t3978 * t3979 * t221 * t22809;
    let t86222 = t3989 * t22815;
    (t86205, t86208, t86212, t86220, t86222)
}
