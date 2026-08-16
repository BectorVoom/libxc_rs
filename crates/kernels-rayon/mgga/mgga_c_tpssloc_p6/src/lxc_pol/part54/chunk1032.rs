//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1032/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1032(t22833: f64, t5293: f64, t5303: f64, t1351: f64, t16311: f64, t3788: f64, t6936: f64, t16306: f64, t550: f64, t1339: f64, t22856: f64, t22859: f64, t22860: f64, t22864: f64, t22868: f64, t26306: f64, t26310: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26312 = t22833 * t5293;
    let t26314 = t22833 * t5303;
    let t26318 = t16311 * t1351;
    let t26319 = t3788 * t26318;
    let t26320 = t6936 * t26319;
    let t26322 = t16306 * t550;
    let t26323 = t1339 * t26322;
    let t26324 = t6936 * t26323;
    let t26326 = t26306 / 384.0_f64 + t26310 / 768.0_f64 - t26312 / 1536.0_f64 + t26314 / 384.0_f64 + 0.33643963411783659045e-4_f64 * t22856 + t22859 - 7.0_f64 / 2304.0_f64 * t22860 + t22864 + t22868 + 0.40372756094140390854e-3_f64 * t26320 - 0.20186378047070195427e-3_f64 * t26324;
    (t26312, t26314, t26318, t26320, t26322, t26324, t26326)
}
