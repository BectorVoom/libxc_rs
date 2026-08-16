//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1280/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1280(t3204: f64, t7125: f64, t11788: f64, t1972: f64, t11782: f64, t1007: f64, t25532: f64, t3080: f64, t7106: f64, t1028: f64, t10344: f64, t11944: f64, t136: f64, t25490: f64, t25495: f64, t3208: f64, t3220: f64, t343: f64, t348: f64, t93713: f64, t93715: f64, t93718: f64, t93720: f64, t93722: f64, t93725: f64) -> f64 {
    let t93728 = t3204 * t7125;
    let t93731 = t11788 * t1972;
    let t93736 = t11782 * t1972;
    let t93743 = t25532 * t1007;
    let t93745 = t7106 * t3080;
    let t93747 = -0.12862205435420921092e-2_f64 * t25490 * t3220 + 0.17149607247227894789e-2_f64 * t93713 - 0.43445671692977333464e-1_f64 * t93715 * t1028 + 0.91464571985215438873e-2_f64 * t93718 + 0.28582678745379824648e-3_f64 * t93720 + 0.13719685797782315831e-1_f64 * t93722 * t1028 - 0.25724410870841842183e-2_f64 * t93725 * t11944 - 0.13719685797782315831e-1_f64 * t93728 * t3208 + 0.25724410870841842183e-2_f64 * t93731 * t3208 + 0.68598428988911579154e-2_f64 * t25495 * t3220 - 0.12862205435420921092e-2_f64 * t93736 * t1028 - 77.0_f64 / 162.0_f64 * t10344 * t343 * t136 * t348 + 11.0_f64 / 108.0_f64 * t93743 + t93745 / 54.0_f64;
    t93747
}
