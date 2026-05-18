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
    let t15466 = t15218 / F::new(384.0) + t15220 / F::new(48.0) - t15222 / F::new(48.0) + t15224 / F::new(384.0) + t15226 / F::new(48.0) - t15228 / F::new(24.0) - t15230 / F::new(384.0) + t15232 / F::new(128.0) - t15234 / F::new(384.0) + t15236 / F::new(12.0) - t15238 / F::new(12.0) - t15241 / F::new(48.0) - t15243 / F::new(384.0) + F::new(7.0) / F::new(288.0) * t14506 - F::new(7.0) / F::new(72.0) * t14520 + t15245 / F::new(48.0);
    (t15443, t15445, t15466)
}
