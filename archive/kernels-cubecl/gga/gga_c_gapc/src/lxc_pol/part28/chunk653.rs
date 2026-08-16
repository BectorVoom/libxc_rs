//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 653/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk653<F: Float>(t3784: F, t3789: F, t2660: F, t3717: F, t2767: F, t3636: F, t3641: F, t3647: F, t3653: F, t209: F, t1049: F, t3480: F) -> (F, F, F, F, F, F) {
    let t3790 = t3784 * t3789;
    let t3792 = t2660 * t3717;
    let t3793 = t3792 * t2767;
    let t3800 = F::cast_from(0.12147342662753799615e-3_f64) * t3636;
    let t3801 = F::cast_from(0.86898242813537603825e-4_f64) * t3641;
    let t3802 = F::cast_from(0.2530696388073708253e-5_f64) * t3647;
    let t3804 = t3800 - t3801 - t3802 + F::cast_from(0.54311401758461002391e-5_f64) * t3653;
    let t3805 = t3804 * t209;
    let t3806 = t3480 * t1049;
    (t3790, t3792, t3793, t3804, t3805, t3806)
}
