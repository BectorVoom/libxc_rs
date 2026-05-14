//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1147/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1147<F: Float>(t3080: F, t7106: F, t1028: F, t10344: F, t11944: F, t136: F, t25490: F, t25495: F, t3208: F, t3220: F, t343: F, t348: F, t93713: F, t93715: F, t93718: F, t93720: F, t93722: F, t93725: F, t93728: F, t93731: F, t93736: F, t93743: F) -> (F,) {
    let t93745 = t7106 * t3080;
    let t93747 = -0.12862205435420921092e-2 * t25490 * t3220 + 0.17149607247227894789e-2 * t93713 - 0.43445671692977333464e-1 * t93715 * t1028 + 0.91464571985215438873e-2 * t93718 + 0.28582678745379824648e-3 * t93720 + 0.13719685797782315831e-1 * t93722 * t1028 - 0.25724410870841842183e-2 * t93725 * t11944 - 0.13719685797782315831e-1 * t93728 * t3208 + 0.25724410870841842183e-2 * t93731 * t3208 + 0.68598428988911579154e-2 * t25495 * t3220 - 0.12862205435420921092e-2 * t93736 * t1028 - 77.0 / 162.0 * t10344 * t343 * t136 * t348 + 11.0 / 108.0 * t93743 + t93745 / 54.0;
    (t93747,)
}
