//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1196/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1196(t11696: f64, t40075: f64, t10710: f64, t10728: f64, t27955: f64, t11699: f64, t39961: f64, t3281: f64, t9236: f64, t3606: f64, t39840: f64, t7624: f64) -> (f64, f64, f64, f64, f64) {
    let t43281 = t40075 * t11696;
    let t43284 = t10728 * t10710 * t27955;
    let t43286 = t39961 * t11699;
    let t43288 = t3281 * t9236;
    let t43291 = t39840 * t3606 * t7624;
    (t43281, t43284, t43286, t43288, t43291)
}
