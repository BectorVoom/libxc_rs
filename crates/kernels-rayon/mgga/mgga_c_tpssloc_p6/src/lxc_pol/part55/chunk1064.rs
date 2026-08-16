//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1064/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1064(t31136: f64, t31219: f64, t533: f64, t1390: f64, t1983: f64, t30991: f64, t6534: f64, t8601: f64, t2314: f64, t8326: f64, t5113: f64, t6876: f64, t8494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31220 = t31136 + t31219;
    let t31221 = t533 * t31220;
    let t31222 = t31221 * t1390;
    let t31223 = t1983 * t31222;
    let t31233 = 2.0_f64 * t30991;
    let t31235 = 4.0_f64 * t8601 * t6534;
    let t31236 = t2314 * t8326;
    let t31237 = 2.0_f64 * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = 2.0_f64 * t31238;
    let t31249 = t6876 * t8494;
    (t31220, t31221, t31222, t31223, t31233, t31235, t31237, t31239, t31249)
}
