//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 511/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk511<F: Float>(t2092: F, t2093: F, t2095: F, t3139: F, t3497: F, t3500: F, t3503: F, t3507: F, t3510: F, t3513: F, t3515: F, t3520: F, t3524: F, t462: F, t92: F, t579: F, t91: F) -> (F, F) {
    let t3526 = t2092 + t2093 / 9.0 + t2095 / 3.0 + t3497 / 9.0 - 2.0 / 9.0 * t462 * t3500 + t462 * t3503 / 3.0 + 2.0 / 3.0 * t462 * t3507 - 2.0 / 3.0 * t3139 * t3510 + t3513 / 3.0 + t462 * t3515 / 3.0 + 2.0 * t462 * t3520 - t92 * t3524;
    let t3528 = t91 * t579 * t3526;
    (t3526, t3528)
}
