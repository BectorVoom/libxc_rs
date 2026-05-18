//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1215/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1215<F: Float>(t2313: F, t6638: F, t12076: F, t19714: F, t2118: F, t3074: F, t6179: F, t6183: F, t2134: F, t20886: F, t343: F, t6345: F, t814: F) -> (F, F, F, F, F) {
    let t21570 = t2313 * t6638;
    let t21577 = F::new(7.0) / F::new(48.0) * t3074 * t2118 * t19714 * t12076;
    let t21578 = t6183 * t6179;
    let t21579 = t2134 * t21578;
    let t21580 = F::new(7.0) / F::new(24.0) * t21579;
    let t21581 = t20886 * t343;
    let t21586 = t6345 * t814;
    (t21570, t21577, t21580, t21581, t21586)
}
