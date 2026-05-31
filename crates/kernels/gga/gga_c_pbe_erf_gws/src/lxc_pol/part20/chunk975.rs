//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 975/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk975<F: Float>(t7845: F, t3454: F, t572: F, t418: F, t5548: F, t587: F, t1017: F, t995: F, t610: F, t7703: F, t1820: F, t2585: F, t7130: F) -> (F, F, F, F) {
    let t11004 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t7845;
    let t11005 = t3454 * t572;
    let t11006 = t11005 * t418;
    let t11007 = t5548 * t11006;
    let t11009 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t11007;
    let t11010 = t995 * t1017;
    let t11011 = t11010 * t610;
    let t11012 = t7703 * t11011;
    let t11014 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1820 * t11012;
    let t11016 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7130 * t2585;
    (t11004, t11009, t11014, t11016)
}
