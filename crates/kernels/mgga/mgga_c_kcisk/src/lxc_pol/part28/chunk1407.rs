//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1407/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1407<F: Float>(t24160: F, t33121: F, t17065: F, t2580: F, t24155: F, t122289: F, t122291: F, t122293: F, t122295: F, t122297: F, t122299: F, t122301: F, t122303: F, t112051: F, t35301: F, t24208: F) -> (F, F, F, F, F, F) {
    let t122305 = t33121 * t24160;
    let t122307 = t17065 * t2580;
    let t122309 = t33121 * t24155;
    let t122311 = 11.0 / 27.0 * t122289 + t122291 / 6.0 - t122293 / 48.0 + 2.0 / 9.0 * t122295 - t122297 / 9.0 + t122299 / 432.0 - t122301 / 9.0 + t122303 / 36.0 + t122305 / 288.0 + t122307 / 64.0 + t122309 / 96.0;
    let t122313 = t112051 * t35301;
    let t122315 = t33121 * t24208;
    (t122305, t122307, t122309, t122311, t122313, t122315)
}
