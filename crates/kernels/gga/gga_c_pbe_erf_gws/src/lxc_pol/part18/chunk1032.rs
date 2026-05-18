//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1032/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1032<F: Float>(t11557: F, t820: F, t9441: F, t3257: F, t346: F, t3747: F, t1114: F, t2150: F, t3134: F, t9108: F, t9111: F, t3757: F, t810: F) -> (F, F, F, F, F, F) {
    let t11558 = t11557 * t820;
    let t11559 = t9441 * t11558;
    let t11560 = t3257 * t11559;
    let t11563 = t3747 * t346;
    let t11564 = t1114 * t11563;
    let t11566 = t11564 * t2150 / F::new(48.0);
    let t11568 = t9108 * t3134 / F::new(48.0);
    let t11570 = t9111 * t3134 / F::new(48.0);
    let t11571 = t3757 * t810;
    (t11559, t11560, t11566, t11568, t11570, t11571)
}
