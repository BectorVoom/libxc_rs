//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 951/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk951<F: Float>(t10722: F, t661: F, t7216: F, t1620: F, t2576: F, t7527: F, t2612: F, t2667: F, t2674: F, t2680: F, t3403: F, t7011: F) -> (F, F, F, F, F, F) {
    let t10723 = t10722 * t661;
    let t10724 = t7216 * t10723;
    let t10726 = F::new(16.0) / F::new(15.0) * t1620 * t10724;
    let t10728 = F::new(16.0) / F::new(45.0) * t7527 * t2576;
    let t10730 = F::new(8.0) / F::new(45.0) * t2612 * t2667;
    let t10732 = F::new(16.0) / F::new(45.0) * t2612 * t2674;
    let t10734 = F::new(8.0) / F::new(27.0) * t2612 * t2680;
    let t10736 = F::new(16.0) / F::new(45.0) * t7011 * t3403;
    (t10726, t10728, t10730, t10732, t10734, t10736)
}
