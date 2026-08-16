//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3536/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3536(t19680: f64, t4786: f64, t1045: f64, t11660: f64, t11703: f64, t11774: f64, t15584: f64, t15689: f64, t15691: f64, t15700: f64, t15701: f64, t15926: f64, t15968: f64, t16040: f64, t16222: f64, t19622: f64, t19700: f64, t19985: f64, t19992: f64, t20040: f64, t42695: f64, t43066: f64, t43082: f64, t43285: f64, t4583: f64, t4892: f64, t53545: f64, t53585: f64, t54991: f64, t55209: f64, t6092: f64, t6273: f64, t999: f64) -> f64 {
    let t67120 = t19680 * t4786;
    let t67143 = -0.57165357490759649296e-3_f64 * t15689 * t53545 * t19985 - 0.57165357490759649296e-3_f64 * t11774 * t15691 * t1045 * t4583 * t999 - 0.28582678745379824648e-3_f64 * t11774 * t15584 * t19700 * t4786 - 0.11433071498151929859e-2_f64 * t15700 * t53545 * t19992 + 0.30488190661738479624e-2_f64 * t43066 * t20040 - 0.57165357490759649296e-3_f64 * t15700 * t15701 * t67120 + 0.47637797908966374414e-3_f64 * t15700 * t16222 * t67120 - 0.14481890564325777821e-1_f64 * t42695 * t6273 - 0.85748036236139473944e-3_f64 * t15926 * t16040 + 0.17149607247227894789e-2_f64 * t43285 * t19622 - 0.11433071498151929859e-2_f64 * t43082 * t55209 * t11660 * t53585 * t999 + 0.47637797908966374414e-3_f64 * t4892 * t11703 * t6092 * t15968 - 0.57165357490759649296e-3_f64 * t54991;
    t67143
}
