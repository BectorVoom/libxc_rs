//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 731/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk731<F: Float>(t11514: F, t254: F, t2157: F, t1105: F, t816: F, t1109: F, t346: F, t3747: F, t1114: F, t2319: F, t3863: F, t3703: F, t5: F, param_a_c: F) -> (F, F, F, F, F, F, F, F) {
    let t11539 = t254 * t11514;
    let t11540 = t2157 * param_a_c;
    let t11551 = t816 * t1105;
    let t11557 = t816 * t1109;
    let t11563 = t3747 * t346;
    let t11564 = t1114 * t11563;
    let t11581 = t2319 * t3863;
    let t11583 = t5 * t3703;
    (t11539, t11540, t11551, t11557, t11563, t11564, t11581, t11583)
}
