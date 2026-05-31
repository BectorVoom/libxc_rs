//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 772/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk772<F: Float>(t2615: F, t3504: F, t1044: F, t3390: F, t5110: F, t186: F, t211: F, t1017: F, t3454: F, t5176: F, t185: F, t10416: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12582 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2615 * t3504;
    let t12583 = t3390 * t1044;
    let t12584 = t5110 * t12583;
    let t12585 = t186 * t12584;
    let t12587 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t211 * t12585;
    let t12588 = t3454 * t1017;
    let t12589 = t5176 * t12588;
    let t12590 = t186 * t12589;
    let t12592 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t185 * t12590;
    let t12593 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t10416;
    (t12582, t12583, t12584, t12585, t12587, t12588, t12589, t12590, t12592, t12593)
}
