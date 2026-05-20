//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1969/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1969<F: Float>(t114: F, t109367: F, t108138: F, t96187: F, t96236: F, t30256: F, t689: F, t25904: F, t102081: F, t102084: F, t102086: F, t102090: F, t102093: F, t102096: F, t102098: F, t102101: F, t102104: F, t102113: F, t96197: F) -> (F, F, F) {
    let t115 = F::new(1.0) < t114;
    let t109368 = piecewise3::<F>(t115, F::new(0.0), t109367);
    let t109391 = t96187 * t108138;
    let t109393 = t96236 * t108138;
    let t109396 = t30256 * t689;
    let t109397 = t25904 * t109396;
    let t109399 = t102081 - t102084 - t102086 - t102090 + t102093 + t102096 - t102098 + F::cast_from(0.3427046870806409921e-2_f64) * t102101 - t102104 + F::cast_from(0.28912093960683998207e-1_f64) * t109391 - F::cast_from(0.51405703062096148813e-1_f64) * t109393 + F::cast_from(0.73171657588172351096e-2_f64) * t96197 + t102113 - F::cast_from(0.14456046980341999104e-1_f64) * t109397;
    (t109368, t109396, t109399)
}
