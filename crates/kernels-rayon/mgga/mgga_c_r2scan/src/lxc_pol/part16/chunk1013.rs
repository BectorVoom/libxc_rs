//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1013/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1013(t12547: f64, t1592: f64, t269: f64, t3177: f64, t1060: f64, t783: f64, t10835: f64, t10843: f64, t10854: f64, t10864: f64, t10867: f64, t10902: f64, t11817: f64, t12192: f64, t12193: f64, t12534: f64, t12536: f64, t12539: f64, t12541: f64, t12544: f64) -> (f64, f64) {
    let t12548 = t1592 * t12547;
    let t12550 = t3177 * t269;
    let t12552 = t783 * t12550 * t1060;
    let t12554 = -0.43663693315433241792e-2_f64 * t12534 + 0.43663693315433241792e-2_f64 * t12536 + 0.21831846657716620896e-2_f64 * t12539 + 0.43341108700271342816e-1_f64 * t12541 - 0.13099107994629972538e-1_f64 * t12544 + t10835 + t10843 + 0.47609969197673950972e-2_f64 * t11817 - t10854 + t10864 + t10867 + t12192 + t12193 + 0.2600466522016280569e0_f64 * t12548 - t10902 - 0.21831846657716620896e-2_f64 * t12552;
    (t12550, t12554)
}
