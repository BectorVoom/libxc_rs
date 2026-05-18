//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 804/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk804<F: Float>(t101: F, t12989: F, t12436: F, t12438: F, t12442: F, t12446: F, t12448: F, t12450: F, t12454: F, t12488: F, t12521: F, t12524: F, t12525: F, t12530: F, t4872: F, t4876: F, t4910: F, t6998: F, t7075: F) -> (F, F) {
    let t12990 = t101 * t12989;
    let t13005 = -t4872 + F::new(2.0) / F::new(45.0) * t6998 + t4876 + t12436 - t12438 - t12442 - t12446 + t12448 + t12450 + t12454 + t4910 + t12488 + t12521 + F::new(4.0) / F::new(3.0) * t7075 + t12524 - t12525 + t12530;
    (t12990, t13005)
}
