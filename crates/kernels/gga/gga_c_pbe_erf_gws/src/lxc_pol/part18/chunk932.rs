//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 932/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk932<F: Float>(t11564: F, t2150: F, t3134: F, t9108: F, t9111: F, t3757: F, t810: F, t3258: F, t2255: F, t2157: F, t3165: F, t3219: F, t3235: F, t2319: F, t3863: F, t3703: F, t5: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11566 = t11564 * t2150 / 48.0;
    let t11568 = t9108 * t3134 / 48.0;
    let t11570 = t9111 * t3134 / 48.0;
    let t11571 = t3757 * t810;
    let t11572 = t3258 * t11571;
    let t11573 = t2255 * t11572;
    let t11576 = t2157 * t3165;
    let t11578 = t3235 * t3219 * t11576;
    let t11581 = t2319 * t3863;
    let t11583 = t5 * t3703;
    (t11566, t11568, t11570, t11572, t11573, t11576, t11578, t11581, t11583)
}
