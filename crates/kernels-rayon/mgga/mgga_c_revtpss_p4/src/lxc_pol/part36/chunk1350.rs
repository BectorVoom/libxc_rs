//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1350/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1350(t104636: f64, t104658: f64, t104703: f64, t104752: f64, t104758: f64, t104762: f64, t112252: f64, t112339: f64, t1797: f64, t24605: f64, t24664: f64, t24706: f64, t24753: f64, t24773: f64, t26870: f64, t29040: f64, t29083: f64, t6619: f64, t6631: f64, t6635: f64, t6673: f64, t6690: f64, t7618: f64, t97211: f64) -> f64 {
    let t116109 = -0.13719685797782315831e-1_f64 * t104758 * t6631 + 0.68598428988911579154e-2_f64 * t104762 * t6635 + 0.25724410870841842183e-2_f64 * t97211 * t24664 - 0.17149607247227894789e-2_f64 * t29040 * t24605 - 0.91464571985215438873e-2_f64 * t104636 * t6619 + 0.17149607247227894789e-2_f64 * t104752 * t6619 + 0.12862205435420921092e-2_f64 * t112339 * t1797 - 0.13719685797782315831e-1_f64 * t112252 * t1797 - 0.25724410870841842183e-2_f64 * t104703 * t6690 - 0.12862205435420921092e-2_f64 * t26870 * t24753 - 0.12862205435420921092e-2_f64 * t26870 * t24706 - 0.7622047665434619906e-2_f64 * t29083 * t6673 + 0.19055119163586549765e-3_f64 * t104658 + 0.42874018118069736972e-3_f64 * t7618 * t24773;
    t116109
}
