//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 531/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk531<F: Float>(t2653: F, t336: F, t714: F, t1062: F, t723: F, t181: F, t562: F, t184: F, t997: F, t1879: F, t1676: F, t1027: F, t661: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2654 = t2653 * t336;
    let t2655 = t2654 * t714;
    let t2657 = t1062 * t723;
    let t2659 = t562 * t181;
    let t2660 = t2659 * t184;
    let t2662 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2660 * t997;
    let t2664 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1879 * t997;
    let t2665 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1676;
    let t2666 = t1027 * t661;
    (t2654, t2655, t2657, t2659, t2660, t2662, t2664, t2665, t2666)
}
