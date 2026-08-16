//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1927/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1927<F: Float>(t14991: F, t95936: F, t7407: F, t99373: F, t2435: F, t28390: F, t102993: F, t25411: F, t2470: F, t28359: F, t7064: F, t7997: F, t822: F) -> (F, F, F, F, F, F, F) {
    let t103396 = t95936 * t14991;
    let t103399 = F::cast_from(0.25702851531048074406e-1_f64) * t99373 * t7407;
    let t103400 = t2435 * t28390;
    let t103404 = t25411 * t102993;
    let t103421 = t28359 * t2470;
    let t103422 = t7064 * t103421;
    let t103424 = t822 * t7997;
    (t103396, t103399, t103400, t103404, t103421, t103422, t103424)
}
