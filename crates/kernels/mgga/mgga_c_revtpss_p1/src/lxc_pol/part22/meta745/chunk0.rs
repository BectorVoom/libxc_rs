//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2814/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2814<F: Float>(t2439: F, t2440: F, t2829: F, t2410: F, t2985: F, t3010: F, t3013: F, t241: F, t281: F, t283: F, t2297: F, t2851: F) -> (F, F, F, F, F, F, F, F) {
    let t41125 = t2439 * t2440 * t2829;
    let t41153 = t2410 * t2410;
    let t41154 = F::new(1.0) / t41153;
    let t41224 = F::new(1.0) / t3010 / t2985;
    let t41234 = t3010 * t3010;
    let t41235 = F::new(1.0) / t41234;
    let t41237 = t3013 * t3013;
    let t41238 = F::new(1.0) / t41237;
    let t41245 = t281 * t241 * t283;
    let t41246 = F::cast_from(0.13490888888888888889e1_f64) * t41245;
    let t41270 = F::new(1.0) / t2851 / t2297;
    (t41125, t41154, t41224, t41235, t41238, t41245, t41246, t41270)
}
