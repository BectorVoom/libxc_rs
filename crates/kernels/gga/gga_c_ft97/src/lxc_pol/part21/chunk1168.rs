//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1168/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1168<F: Float>(t102203: F, t116593: F, t116598: F, t116601: F, t116606: F, t116611: F, t116613: F, t116616: F, t116621: F, t116626: F, t116629: F, t116631: F, t116316: F, t3281: F, t7793: F, t116260: F, t446: F, t7824: F) -> (F, F, F) {
    let t116633 = t102203 - 4.0 / 3.0 * t116593 - 3.0 * t116598 + 4.0 / 9.0 * t116601 + t116606 / 4.0 + t116611 - 4.0 / 3.0 * t116613 - 2.0 / 3.0 * t116616 + t116621 / 2.0 - 3.0 / 8.0 * t116626 + t116629 + 10.0 / 27.0 * t116631;
    let t116635 = t3281 * t7793 * t116316;
    let t116638 = t446 * t7824 * t116260;
    (t116633, t116635, t116638)
}
