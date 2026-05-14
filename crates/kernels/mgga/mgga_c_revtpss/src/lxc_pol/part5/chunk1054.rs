//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1054/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1054<F: Float>(t189: F, t5825: F, t606: F, t4401: F, t10552: F, t10554: F, t14317: F, t18253: F, t18256: F, t18261: F, t18262: F, t18265: F, t18267: F, t18268: F, t18300: F, t18301: F, t1940: F, t2403: F, t4537: F, t4541: F, t4556: F, t775: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F) {
    let t18305 = t189 * t5825;
    let t18306 = t18305 * t606;
    let t18308 = 12.0 * t4401 * t18306;
    let t18309 = -3.0 * t18268 * t2403 * t775 - 2.0 * t1940 * t4537 * t4556 + 12.0 * t18253 * t4541 + 6.0 * t18256 * t4541 - t10552 + t10554 + t14317 + t18261 + t18262 + t18265 + t18267 + t18300 + t18301 + t18308 - t9278 + t9308 + t9316 + t9329 + t9333;
    (t18308, t18309)
}
