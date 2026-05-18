//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 906/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk906<F: Float>(t247: F, t7908: F, t251: F, t2626: F, t5018: F, t1820: F, t1898: F, t2615: F, t1648: F, t2643: F, t1733: F, t2596: F) -> (F, F, F, F, F) {
    let t7909 = t7908 * t247;
    let t7910 = t7909 * t251;
    let t7913 = t5018 * t2626;
    let t7915 = F::new(16.0) / F::new(45.0) * t1820 * t7913;
    let t7917 = F::new(8.0) / F::new(45.0) * t2615 * t1898;
    let t7919 = F::new(16.0) / F::new(135.0) * t1648 * t2643;
    let t7920 = t2596 * t1733;
    (t7910, t7915, t7917, t7919, t7920)
}
