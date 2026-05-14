//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1339/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1339<F: Float>(t3: F, t31582: F, t2178: F, t5883: F, t1518: F, t31370: F, t5920: F, t8295: F, t117: F, t31555: F, t1916: F, t1918: F, t2187: F, t2189: F, t572: F, t573: F, t6941: F, t6945: F, t6948: F, t8377: F, t8383: F, t8386: F) -> (F, F, F, F, F, F, F) {
    let t31583 = t3 * t31582;
    let t31593 = param_d * t31582;
    let t31607 = t5883 * t2178;
    let t31610 = t31370 * t1518;
    let t31613 = t8295 * t5920;
    let t31616 = t117 * t31555;
    let t31619 = 12.0 * t1916 * t8383 + 6.0 * t1916 * t8386 + 6.0 * t1918 * t8377 + 6.0 * t2187 * t6945 + 3.0 * t2187 * t6948 + 3.0 * t2189 * t6941 + t31593 * t573 + 6.0 * t31607 * t572 + 12.0 * t31610 * t572 + 6.0 * t31613 * t572 + 3.0 * t31616 * t572;
    (t31583, t31593, t31607, t31610, t31613, t31616, t31619)
}
