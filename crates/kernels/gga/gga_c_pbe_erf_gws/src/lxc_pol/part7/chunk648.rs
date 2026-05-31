//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 648/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk648<F: Float>(t5058: F, t5096: F, t650: F, t186: F, t211: F, t1672: F, t662: F, t1794: F, t582: F, t648: F, t213: F, t1793: F, t661: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5097 = t5058 + t5096;
    let t5098 = t650 * t5097;
    let t5099 = t186 * t5098;
    let t5101 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t211 * t5099;
    let t5102 = t1672 * t662;
    let t5103 = t211 * t5102;
    let t5104 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5103;
    let t5105 = t582 * t1794;
    let t5106 = t211 * t5105;
    let t5107 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5106;
    let t5108 = t648 * t648;
    let t5109 = F::cast_from(1.0_f64) / t5108;
    let t5110 = t213 * t5109;
    let t5111 = t1793 * t661;
    (t5097, t5098, t5099, t5101, t5102, t5104, t5105, t5107, t5108, t5109, t5110, t5111)
}
