//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 847/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk847<F: Float>(t1764: F, t7148: F, t7051: F, t5218: F, t572: F, t2735: F, t185: F, t1019: F, t1680: F, t1791: F, t2722: F, t661: F) -> (F, F, F, F) {
    let t7149 = t7148 * t1764;
    let t7150 = t7149 * t7051;
    let t7152 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t5218 * t7150;
    let t7153 = t7148 * t572;
    let t7154 = t2735 * t7153;
    let t7156 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t185 * t7154;
    let t7158 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1680 * t1019;
    let t7159 = t1791 * t2722;
    let t7160 = t7159 * t661;
    (t7152, t7156, t7158, t7160)
}
