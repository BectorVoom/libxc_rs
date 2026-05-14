//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1338/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1338<F: Float>(t10199: F, t2339: F, t2: F, t665: F, t101457: F, t116919: F, t116946: F, t1504: F, t1513: F, t2256: F, t2340: F, t2350: F, t28036: F, t31035: F, t31039: F, t31054: F, t31058: F, t31267: F, t31276: F, t31287: F, t4287: F, t658: F, t8258: F, t8259: F, t8267: F, t8268: F) -> (F,) {
    let t117544 = t10199 * t2339;
    let t117545 = t2 * t665;
    let t117560 = -25.0 / 18.0 * t8258 * t31054 * t31267 + 5.0 / 6.0 * t8258 * t8268 * t4287 * t658 + 5.0 / 12.0 * t8258 * t8268 * t1513 * t2256 + 5.0 / 2.0 * t31035 * t31039 * t28036 + 5.0 / 18.0 * t8258 * t31058 * t1513 * t2350 - 5.0 / 4.0 * t31035 * t8268 * t1504 * t2340 - 25.0 / 18.0 * t8258 * t31054 * t31276 + 5.0 / 6.0 * t117544 * t8268 * t117545 + 5.0 / 108.0 * t8267 * t116946 * t1504 * t2350 - 5.0 / 18.0 * t31287 * t31058 * t2 * t658 + 3.0 * t116919 * t8259 * t101457;
    (t117560,)
}
