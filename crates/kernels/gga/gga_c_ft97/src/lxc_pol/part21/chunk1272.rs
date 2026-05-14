//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1272/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1272<F: Float>(t119581: F, t1969: F, t446: F, t16150: F, t95332: F, t9049: F, t119584: F, t3281: F, t119577: F, t9073: F, t95312: F, t39725: F, t119729: F, t119732: F, t119734: F, t119737: F, t119740: F, t119745: F) -> (F, F, F, F, F, F, F, F) {
    let t119748 = t446 * t1969 * t119581;
    let t119750 = t95332 * t16150;
    let t119752 = t446 * t9049 * t119750;
    let t119755 = t3281 * t1969 * t119584;
    let t119758 = t446 * t9073 * t119577;
    let t119760 = t95312 * t16150;
    let t119762 = t446 * t39725 * t119760;
    let t119764 = 2.0 / 3.0 * t119729 + t119732 - t119734 - 4.0 / 3.0 * t119737 + 4.0 / 9.0 * t119740 - 3.0 / 8.0 * t119745 + 2.0 * t119748 - 4.0 / 3.0 * t119752 - 8.0 / 3.0 * t119755 + 4.0 / 3.0 * t119758 + 10.0 / 27.0 * t119762;
    (t119748, t119750, t119752, t119755, t119758, t119760, t119762, t119764)
}
