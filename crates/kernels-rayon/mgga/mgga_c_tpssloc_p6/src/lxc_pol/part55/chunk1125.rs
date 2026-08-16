//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1125/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1125(t24615: f64, t8060: f64, t7300: f64, t1760: f64, t8887: f64, t11606: f64, t1653: f64, t32514: f64, t24601: f64, t1716: f64, t32515: f64, t1238: f64, t2155: f64, t24589: f64, t27406: f64, t27830: f64, t32498: f64, t32542: f64, t4945: f64, t5055: f64, t7283: f64, t7351: f64, t7999: f64, t8061: f64, t8088: f64, t8868: f64, t8872: f64, t8888: f64, t8898: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34322 = t24615 * t8060;
    let t34323 = t7300 * t34322;
    let t34330 = t8887 * t1760;
    let t34331 = t11606 * t34330;
    let t34338 = t32514 * t1653;
    let t34339 = t24601 * t34338;
    let t34349 = t1716 * t32515;
    let t34352 = 0.3289868133696452873e-1_f64 * t7283 * t34323 + 2.0_f64 * t4945 * t8888 + 2.0_f64 * t5055 * t8888 - 6.0_f64 * t1238 * t34331 - 2.0_f64 * t7351 * t8088 + 0.43864908449286038307e-1_f64 * t27406 * t8872 + t32498 + 0.54831135561607547883e-2_f64 * t24589 * t34339 - 2.0_f64 * t27830 * t2155 - t5055 * t8898 + 4.0_f64 * t7351 * t8061 - 0.43864908449286038307e-1_f64 * t7999 * t8868 + 0.16449340668482264365e-1_f64 * t7283 * t34349 - t32542;
    (t34322, t34323, t34331, t34338, t34339, t34349, t34352)
}
