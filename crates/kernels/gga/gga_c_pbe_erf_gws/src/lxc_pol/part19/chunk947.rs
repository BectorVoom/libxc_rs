//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 947/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk947<F: Float>(t10676: F, t5218: F, t572: F, t7514: F, t10392: F, t610: F, t7062: F, t1651: F, t3503: F, t587: F, t2609: F, t7527: F) -> (F, F, F, F) {
    let t10678 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5218 * t10676;
    let t10679 = t7514 * t572;
    let t10681 = t10679 * t10392 * t610;
    let t10683 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7062 * t10681;
    let t10685 = t1651 * t3503;
    let t10686 = t587 * t10685;
    let t10687 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t10686;
    let t10690 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t7527 * t2609;
    (t10678, t10683, t10687, t10690)
}
