//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1069/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1069<F: Float>(t34053: F, t686: F, t72: F, t32474: F, t122034: F, t27341: F, t119915: F, t119937: F, t121902: F, t121914: F, t121921: F, t126214: F, t126226: F, t126232: F, t127704: F, t3140: F, t4469: F, t7073: F, t8477: F, t8652: F) -> (F, F) {
    let t127724 = t34053 * t72 * t686;
    let t127725 = t32474 * t127724;
    let t127727 = t122034 * t27341;
    let t127730 = t119915 + 0.57119737665102352616e0 * t8477 * t4469 * t3140 * t8652 + 0.7437465841810202164e-3 * t126214 - 0.50779446784275991476e-1 * t121902 - 0.34708173928447610099e-2 * t126226 + 0.225875734067843736e-2 * t126232 - 0.14279934416275588154e-1 * t121914 + t119937 + 0.17347256376410398924e1 * t127704 * t7073 + 0.25389723392137995738e-1 * t127725 - 0.28912093960683998207e-1 * t127727 - 0.14279934416275588154e-1 * t121921;
    (t127724, t127730)
}
