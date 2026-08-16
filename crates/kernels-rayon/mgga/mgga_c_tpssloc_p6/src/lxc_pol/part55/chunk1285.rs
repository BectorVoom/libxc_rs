//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1285/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1285(t32514: f64, t8009: f64, t117855: f64, t118059: f64, t118067: f64, t1186: f64, t1238: f64, t15797: f64, t1716: f64, t24615: f64, t24893: f64, t27406: f64, t27411: f64, t27415: f64, t27741: f64, t27760: f64, t27792: f64, t27830: f64, t32480: f64, t32489: f64, t32530: f64, t32543: f64, t3598: f64, t4930: f64, t5055: f64, t7283: f64, t7300: f64, t7301: f64, t7356: f64, t7391: f64, t7392: f64, t8061: f64, t8087: f64, t8088: f64, t8867: f64, t8898: f64) -> f64 {
    let t125662 = t8009 * t32514;
    let t125668 = 2.0_f64 * t5055 * t32489 + 4.0_f64 * t24893 * t8061 - 2.0_f64 * t27792 * t7392 - 0.16449340668482264365e-1_f64 * t7283 * t4930 * t8867 + 4.0_f64 * t27830 * t7356 - 0.16449340668482264365e-1_f64 * t7283 * t32543 * t27415 - 0.54831135561607547883e-2_f64 * t118059 - 0.16449340668482264365e-1_f64 * t7283 * t7300 * t7301 * t27741 + 0.3289868133696452873e-1_f64 * t7283 * t7300 * t24615 * t27760 - t5055 * t32480 - t15797 * t8898 + 0.16449340668482264365e-1_f64 * t7283 * t1716 * t117855 + 4.0_f64 * t1238 * t3598 * t7391 * t8087 + t118067 + 0.3289868133696452873e-1_f64 * t7283 * t32543 * t27411 - 2.0_f64 * t24893 * t8088 + 0.16449340668482264365e-1_f64 * t7283 * t1186 * t125662 + 0.43864908449286038307e-1_f64 * t27406 * t32530;
    t125668
}
