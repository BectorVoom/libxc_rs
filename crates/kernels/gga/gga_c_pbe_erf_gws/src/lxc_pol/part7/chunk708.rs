//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 708/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk708<F: Float>(t1548: F, t156: F, t496: F, t128: F, t5645: F, t10: F, t120: F, t1508: F, t19: F, t5763: F, t1563: F, t5683: F) -> (F, F, F, F, F, F, F) {
    let t5787 = t156 * t1548;
    let t5788 = t496 * t5787;
    let t5790 = t128 * t5645;
    let t5791 = t10 * t5790;
    let t5795 = t1508 * t120 * t19;
    let t5796 = t5795 * t5763;
    let t5797 = F::cast_from(0.97434166666666666666e0_f64) * t5796;
    let t5798 = t1563 * t5683;
    (t5787, t5788, t5790, t5791, t5795, t5797, t5798)
}
