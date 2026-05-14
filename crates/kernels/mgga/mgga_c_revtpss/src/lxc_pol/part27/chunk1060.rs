//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1060/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1060<F: Float>(t2430: F, t605: F, t2257: F, t775: F, t2394: F, t11054: F, t30: F, t10489: F, t198: F, t206: F, t7086: F, t10627: F, t25373: F, t25392: F, t25386: F, t268: F, t41040: F, t837: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t92795 = t605 * t2430;
    let t92799 = t2257 * t775;
    let t92806 = t605 * t2394;
    let t92810 = t30 * t11054;
    let t92814 = t30 * t10489;
    let t92819 = t198 * t206 * t7086;
    let t92822 = t198 * t10627;
    let t92837 = t25373 * t25392;
    let t92838 = t25386 * t92837;
    let t92840 = t268 * t41040 * t837;
    (t92795, t92799, t92806, t92810, t92814, t92819, t92822, t92837, t92838, t92840)
}
