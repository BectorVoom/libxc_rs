//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1191/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1191<F: Float>(t101690: F, t116593: F, t116598: F, t116601: F, t116606: F, t116611: F, t116613: F, t116616: F, t116621: F, t116626: F, t116628: F, t116631: F, t101691: F, t101709: F, t116635: F, t116638: F, t116641: F, t116645: F, t116650: F, t116655: F, t116659: F, t116661: F, t116666: F, t116670: F) -> (F, F) {
    let t117166 = t101690 - 4.0 / 9.0 * t116593 - t116598 + 4.0 / 27.0 * t116601 + t116606 / 12.0 + t116611 / 3.0 - 4.0 / 9.0 * t116613 - 2.0 / 9.0 * t116616 + t116621 / 6.0 - t116626 / 8.0 + 4.0 / 27.0 * t116628 + 10.0 / 81.0 * t116631;
    let t117178 = 8.0 / 27.0 * t116635 - 4.0 / 9.0 * t116638 - 2.0 / 9.0 * t116641 + 4.0 / 81.0 * t101691 - 2.0 / 9.0 * t116645 + 5.0 / 16.0 * t116650 + t116655 / 4.0 - t116659 / 4.0 + t116661 / 9.0 - t116666 / 6.0 + 4.0 / 3.0 * t116670 - t101709;
    (t117166, t117178)
}
