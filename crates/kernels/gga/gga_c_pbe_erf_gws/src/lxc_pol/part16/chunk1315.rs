//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1315/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1315<F: Float>(t22509: F, t4218: F, t14906: F, t4414: F, t1144: F, t14186: F, t859: F, t53334: F, t14945: F, t9270: F, t15022: F, t20154: F, t3067: F, t4216: F, t810: F) -> (F, F, F, F, F, F, F) {
    let t55059 = t22509 * t4218;
    let t55062 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4414 * t14906;
    let t55065 = t859 * t1144 * t14186;
    let t55074 = F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t53334;
    let t55077 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9270 * t14945;
    let t55087 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t4414 * t15022;
    let t55090 = t20154 * t3067 * t4216 * t810;
    (t55059, t55062, t55065, t55074, t55077, t55087, t55090)
}
