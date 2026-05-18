//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1350/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1350<F: Float>(t104636: F, t104658: F, t104703: F, t104752: F, t104758: F, t104762: F, t112252: F, t112339: F, t1797: F, t24605: F, t24664: F, t24706: F, t24753: F, t24773: F, t26870: F, t29040: F, t29083: F, t6619: F, t6631: F, t6635: F, t6673: F, t6690: F, t7618: F, t97211: F) -> F {
    let t116109 = -F::new(0.13719685797782315831e-1) * t104758 * t6631 + F::new(0.68598428988911579154e-2) * t104762 * t6635 + F::new(0.25724410870841842183e-2) * t97211 * t24664 - F::new(0.17149607247227894789e-2) * t29040 * t24605 - F::new(0.91464571985215438873e-2) * t104636 * t6619 + F::new(0.17149607247227894789e-2) * t104752 * t6619 + F::new(0.12862205435420921092e-2) * t112339 * t1797 - F::new(0.13719685797782315831e-1) * t112252 * t1797 - F::new(0.25724410870841842183e-2) * t104703 * t6690 - F::new(0.12862205435420921092e-2) * t26870 * t24753 - F::new(0.12862205435420921092e-2) * t26870 * t24706 - F::new(0.7622047665434619906e-2) * t29083 * t6673 + F::new(0.19055119163586549765e-3) * t104658 + F::new(0.42874018118069736972e-3) * t7618 * t24773;
    t116109
}
