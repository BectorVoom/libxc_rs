//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3536/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3536<F: Float>(t19680: F, t4786: F, t1045: F, t11660: F, t11703: F, t11774: F, t15584: F, t15689: F, t15691: F, t15700: F, t15701: F, t15926: F, t15968: F, t16040: F, t16222: F, t19622: F, t19700: F, t19985: F, t19992: F, t20040: F, t42695: F, t43066: F, t43082: F, t43285: F, t4583: F, t4892: F, t53545: F, t53585: F, t54991: F, t55209: F, t6092: F, t6273: F, t999: F) -> F {
    let t67120 = t19680 * t4786;
    let t67143 = -F::cast_from(0.57165357490759649296e-3_f64) * t15689 * t53545 * t19985 - F::cast_from(0.57165357490759649296e-3_f64) * t11774 * t15691 * t1045 * t4583 * t999 - F::cast_from(0.28582678745379824648e-3_f64) * t11774 * t15584 * t19700 * t4786 - F::cast_from(0.11433071498151929859e-2_f64) * t15700 * t53545 * t19992 + F::cast_from(0.30488190661738479624e-2_f64) * t43066 * t20040 - F::cast_from(0.57165357490759649296e-3_f64) * t15700 * t15701 * t67120 + F::cast_from(0.47637797908966374414e-3_f64) * t15700 * t16222 * t67120 - F::cast_from(0.14481890564325777821e-1_f64) * t42695 * t6273 - F::cast_from(0.85748036236139473944e-3_f64) * t15926 * t16040 + F::cast_from(0.17149607247227894789e-2_f64) * t43285 * t19622 - F::cast_from(0.11433071498151929859e-2_f64) * t43082 * t55209 * t11660 * t53585 * t999 + F::cast_from(0.47637797908966374414e-3_f64) * t4892 * t11703 * t6092 * t15968 - F::cast_from(0.57165357490759649296e-3_f64) * t54991;
    t67143
}
