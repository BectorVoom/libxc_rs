//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 557/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk557<F: Float>(t1173: F, t7764: F, t3651: F, t7757: F, t1180: F, t3661: F, t7736: F, t26: F, t1186: F, t7740: F, t7744: F, t3646: F, t3658: F, t5668: F, t5736: F, t7738: F, t7742: F, t7746: F, t7758: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7765 = t1173 * t7764;
    let t7771 = t3651 * t7757;
    let t7773 = t1180 * t7764;
    let t7776 = t3661 * t7736;
    let t7777 = t26 * t7776;
    let t7779 = t1186 * t7740;
    let t7780 = t26 * t7779;
    let t7782 = t1186 * t7744;
    let t7783 = t26 * t7782;
    let t7785 = -F::new(0.9494625e0) * t7758 + F::new(0.1898925e1) * t7765 + t3646 + F::cast_from(0.19931111111111111111e0_f64) * t5668 - F::cast_from(0.19931111111111111111e0_f64) * t7738 + F::cast_from(0.59793333333333333334e0_f64) * t7742 - F::cast_from(0.29896666666666666667e0_f64) * t7746 + F::new(0.15358125e0) * t7771 + F::new(0.3071625e0) * t7773 + t3658 + F::cast_from(0.10954222222222222222e0_f64) * t5736 - F::cast_from(0.27385555555555555556e-1_f64) * t7777 + F::cast_from(0.16431333333333333333e0_f64) * t7780 - F::cast_from(0.82156666666666666667e-1_f64) * t7783;
    (t7765, t7771, t7773, t7776, t7777, t7779, t7780, t7782, t7783, t7785)
}
