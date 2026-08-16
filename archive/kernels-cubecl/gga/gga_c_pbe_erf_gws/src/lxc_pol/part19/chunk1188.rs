//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1188/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1188<F: Float>(t1161: F, t4227: F, t2409: F, t3067: F, t14506: F, t14520: F, t15218: F, t15220: F, t15222: F, t15224: F, t15226: F, t15228: F, t15230: F, t15232: F, t15234: F, t15236: F, t15238: F, t15241: F, t15243: F, t15245: F) -> (F, F, F) {
    let t15443 = t4227 * t1161;
    let t15445 = t2409 * t3067 * t15443;
    let t15466 = t15218 / F::cast_from(384.0_f64) + t15220 / F::cast_from(48.0_f64) - t15222 / F::cast_from(48.0_f64) + t15224 / F::cast_from(384.0_f64) + t15226 / F::cast_from(48.0_f64) - t15228 / F::cast_from(24.0_f64) - t15230 / F::cast_from(384.0_f64) + t15232 / F::cast_from(128.0_f64) - t15234 / F::cast_from(384.0_f64) + t15236 / F::cast_from(12.0_f64) - t15238 / F::cast_from(12.0_f64) - t15241 / F::cast_from(48.0_f64) - t15243 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14506 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t14520 + t15245 / F::cast_from(48.0_f64);
    (t15443, t15445, t15466)
}
