//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 880/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk880<F: Float>(t1802: F, t2784: F, t610: F, t1885: F, t587: F, t1635: F, t2612: F, t1645: F, t1656: F, t2615: F, t1666: F, t1010: F, t5406: F) -> (F, F, F, F, F, F) {
    let t7589 = t1802 * t2784;
    let t7590 = t7589 * t610;
    let t7591 = t1885 * t7590;
    let t7593 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t587 * t7591;
    let t7595 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2612 * t1635;
    let t7597 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t2612 * t1645;
    let t7599 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2615 * t1656;
    let t7601 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t2615 * t1666;
    let t7603 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5406 * t1010;
    (t7593, t7595, t7597, t7599, t7601, t7603)
}
