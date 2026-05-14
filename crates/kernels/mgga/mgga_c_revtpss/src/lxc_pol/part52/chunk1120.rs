//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1120/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1120<F: Float>(t27888: F, t32716: F, t34236: F, t689: F, t121365: F, t125833: F, t121208: F, t122451: F, t122454: F, t125807: F, t125814: F, t125819: F, t125826: F, t125831: F, t128786: F, t128709: F, t7063: F, t7286: F) -> (F, F, F) {
    let t128788 = t32716 * t27888;
    let t128790 = t34236 * t689;
    let t128791 = t121365 * t128790;
    let t128795 = 0.13223814266738539448e-3 * t125833;
    let t128796 = 0.225875734067843736e-2 * t125807 + t122451 - t121208 + 0.14874931683620404328e-2 * t125814 - t122454 + 0.112937867033921868e-2 * t125819 + 0.25389723392137995738e-1 * t128786 - 0.25702851531048074406e-1 * t128788 - 0.76169170176413987216e-1 * t128791 - 0.26773803678175077509e-3 * t125826 - 0.74374658418102021639e-4 * t125831 + t128795;
    let t128802 = t7063 * t128709 * t7286;
    (t128790, t128796, t128802)
}
