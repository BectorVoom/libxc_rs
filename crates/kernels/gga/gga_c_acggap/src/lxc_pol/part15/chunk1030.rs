//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1030/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1030<F: Float>(t1089: F, t4643: F, t598: F, t8564: F, t2297: F, t8791: F, t13364: F, t33952: F, t2046: F, t336: F, t5506: F, t579: F, t13299: F, t33944: F, t40017: F, t13287: F, t31195: F, t39827: F) -> (F, F, F, F, F, F) {
    let t40436 = t598 * t1089 * t4643 * t8564;
    let t40440 = t2297 * t8791;
    let t40442 = t33952 * t13364 * t40440;
    let t40446 = t2046 * t336 * t579 * t5506;
    let t40450 = t33944 * t13299 * t40017;
    let t40455 = t31195 * t13287 * t39827;
    (t40436, t40440, t40442, t40446, t40450, t40455)
}
