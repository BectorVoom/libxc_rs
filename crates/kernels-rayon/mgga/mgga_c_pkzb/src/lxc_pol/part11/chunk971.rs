//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 971/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk971(t7047: f64, t8850: f64, t8852: f64, t10534: f64, t124: f64, t5028: f64, t5040: f64, t5066: f64, t5069: f64, t5073: f64, t5324: f64, t5333: f64, t5338: f64, t5344: f64) -> (f64, f64, f64, f64, f64) {
    let t10596 = 0.32530743900905219526e-1_f64 * t7047;
    let t10597 = 12.0_f64 * t8850;
    let t10598 = 12.0_f64 * t8852;
    let t10600 = 0.19751673498613801407e-1_f64 * t10534 * t124;
    let t10601 = t10596 + t5028 - t10597 - t10598 + t10600 - t5324 + t5040 + t5066 - t5069 - t5073 + t5333 - t5338 - t5344;
    (t10596, t10597, t10598, t10600, t10601)
}
