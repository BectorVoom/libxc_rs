//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1200/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1200<F: Float>(t30: F, t265: F, t393: F, t114089: F, t113492: F, t1469: F, t2129: F, t22671: F, t30727: F, t45: F, t5825: F, t8161: F, t104636: F, t104658: F, t104703: F, t104752: F, t104758: F, t104762: F, t112252: F, t112339: F, t1797: F, t24605: F, t24664: F, t24706: F, t24753: F, t24773: F, t26870: F, t29040: F, t29083: F, t6619: F, t6631: F, t6635: F, t6673: F, t6690: F, t7618: F, t97211: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t116053 = piecewise3(t394, 0.0, t114089);
    let t116063 = piecewise3(t120, t113492, t116053 * t45 / 2.0 + 3.0 / 2.0 * t30727 * t1469 + 3.0 / 2.0 * t8161 * t5825 + t2129 * t22671 / 2.0);
    let t116109 = -0.13719685797782315831e-1 * t104758 * t6631 + 0.68598428988911579154e-2 * t104762 * t6635 + 0.25724410870841842183e-2 * t97211 * t24664 - 0.17149607247227894789e-2 * t29040 * t24605 - 0.91464571985215438873e-2 * t104636 * t6619 + 0.17149607247227894789e-2 * t104752 * t6619 + 0.12862205435420921092e-2 * t112339 * t1797 - 0.13719685797782315831e-1 * t112252 * t1797 - 0.25724410870841842183e-2 * t104703 * t6690 - 0.12862205435420921092e-2 * t26870 * t24753 - 0.12862205435420921092e-2 * t26870 * t24706 - 0.7622047665434619906e-2 * t29083 * t6673 + 0.19055119163586549765e-3 * t104658 + 0.42874018118069736972e-3 * t7618 * t24773;
    (t116063, t116109)
}
