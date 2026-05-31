//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 843/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk843<F: Float>(t2677: F, t7093: F, t639: F, t1416: F, t2678: F, t5103: F, t1004: F, t1678: F, t184: F, t199: F, t1022: F, t5212: F) -> (F, F, F, F, F, F) {
    let t7094 = t2677 * t7093;
    let t7096 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t639 * t7094;
    let t7097 = t2678 * t1416;
    let t7098 = t2677 * t7097;
    let t7100 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t639 * t7098;
    let t7101 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t5103;
    let t7102 = t1678 * t1004;
    let t7103 = t7102 * t184;
    let t7105 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t7103 * t199;
    let t7106 = t5212 * t1022;
    (t7096, t7097, t7100, t7101, t7105, t7106)
}
