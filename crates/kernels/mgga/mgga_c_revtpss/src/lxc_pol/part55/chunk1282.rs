//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1282/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1282<F: Float>(t128331: F, t128332: F, t128333: F, t128335: F, t128337: F, t128338: F, t128339: F, t128340: F, t128341: F, t128354: F, t128356: F, t128357: F, t128358: F, t128360: F, t128361: F, t128362: F, t128363: F, t128367: F, t130898: F, t33644: F) -> F {
    let t130928 = F::new(2.0) * t128331 + F::new(2.0) * t128332 + F::new(2.0) * t128333 + F::new(2.0) * t128335 + F::new(2.0) * t128337 + F::new(2.0) * t128338 + F::new(2.0) * t128339 + F::new(2.0) * t128340 + F::new(2.0) * t128341 + F::new(2.0) * t128354 + F::new(2.0) * t128356 + F::new(2.0) * t128357 + F::new(2.0) * t128358 + F::new(2.0) * t128360 + F::new(2.0) * t128361 + F::new(2.0) * t128362 + F::new(2.0) * t128363 + t128367 + t130898 + t33644;
    t130928
}
