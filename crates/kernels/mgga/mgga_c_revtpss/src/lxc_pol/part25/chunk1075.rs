//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1075/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1075<F: Float>(t26015: F, t1941: F, t550: F, t3946: F, t1389: F, t25240: F, t3964: F, t7262: F, t820: F, t843: F, t1401: F, t241: F) -> (F, F, F, F, F, F) {
    let t26016 = 0.11433071498151929859e-3 * t26015;
    let t26017 = t1941 * t550;
    let t26018 = t26017 * t3946;
    let t26021 = t3964 * t25240 * t1389;
    let t26022 = 0.90357964994909313586e-5 * t26021;
    let t26024 = t820 * t7262 * t843;
    let t26025 = t26024 * t1401;
    let t26028 = t820 * t7262 * t241;
    (t26016, t26018, t26022, t26024, t26025, t26028)
}
