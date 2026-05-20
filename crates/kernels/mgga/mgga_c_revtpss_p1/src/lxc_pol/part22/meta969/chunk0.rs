//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3234/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3234<F: Float>(t4401: F, t606: F, t61303: F, t50865: F, t50868: F, t14325: F, t18559: F, t14369: F, t4186: F, t40156: F, t11084: F, t2403: F, t5962: F, t61292: F, t61293: F, t61295: F, t61297: F, t61300: F, t61302: F) -> (F, F, F, F, F, F, F) {
    let t61305 = t4401 * t61303 * t606;
    let t61306 = F::new(24.0) * t61305;
    let t61310 = F::new(16.0) * t50865;
    let t61311 = F::new(48.0) * t50868;
    let t61313 = F::new(48.0) * t14325 * t18559;
    let t61315 = t4401 * t14369 * t4186;
    let t61316 = F::new(48.0) * t61315;
    let t61317 = F::cast_from(0.17315859105681463759e2_f64) * t40156;
    let t61318 = -F::new(3.0) * t11084 * t2403 * t5962 - t61292 - t61293 - t61295 - t61297 + t61300 + t61302 + t61306 + t61310 + t61311 + t61313 + t61316 - t61317;
    (t61306, t61310, t61311, t61313, t61316, t61317, t61318)
}
