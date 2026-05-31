//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1286/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1286<F: Float>(t125507: F, t128303: F, t128305: F, t128306: F, t128317: F, t128319: F, t128321: F, t128324: F, t128326: F, t27123: F, t28709: F, t33311: F, t4248: F, t651: F, t7373: F, t8233: F, t8764: F, t8892: F) -> F {
    let t130975 = -F::cast_from(2.0_f64) * t651 * t7373 * t8233 - F::cast_from(2.0_f64) * t27123 * t8892 - t28709 * t8764 - F::cast_from(2.0_f64) * t33311 * t4248 - t125507 - F::cast_from(2.0_f64) * t128303 - F::cast_from(2.0_f64) * t128305 - F::cast_from(2.0_f64) * t128306 - t128317 - t128319 - t128321 - t128324 + t128326;
    t130975
}
