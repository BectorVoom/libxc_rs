//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 867/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk867(t2900: f64, t9314: f64, t302: f64, t5955: f64, t759: f64, t9282: f64, t2901: f64, t2104: f64, t2887: f64, t2899: f64, t2922: f64, t3647: f64, t5984: f64, t7664: f64, t7712: f64, t7715: f64, t7736: f64, t7742: f64, t9270: f64, t9274: f64, t9279: f64, t9284: f64, t9289: f64, t9293: f64, t9298: f64, t9302: f64, t9308: f64, t9311: f64) -> (f64, f64, f64, f64, f64) {
    let t9315 = t2900 * t9314;
    let t9316 = t302 * t9315;
    let t9319 = t5955 * t759;
    let t9320 = t9282 * t9319;
    let t9321 = t302 * t9320;
    let t9324 = t9282 * t2901;
    let t9325 = t302 * t9324;
    let t9328 = -0.42874018118069736972e-3_f64 * t2104 * t9270 + 0.42874018118069736972e-3_f64 * t2922 * t9274 - 0.42874018118069736972e-3_f64 * t2922 * t9279 + 0.21437009059034868486e-3_f64 * t7664 * t9284 + 0.12862205435420921092e-2_f64 * t2104 * t9289 + 0.85748036236139473944e-3_f64 * t2922 * t9293 - t2887 * t9298 / 16.0_f64 + t2887 * t9302 / 24.0_f64 + 0.45732285992607719437e-2_f64 * t5984 * t3647 - 0.57165357490759649296e-3_f64 * t9308 - 0.85748036236139473944e-3_f64 * t2899 * t9311 + 0.85748036236139473944e-3_f64 * t2899 * t9316 + 0.12862205435420921092e-2_f64 * t7736 * t9321 - 0.12862205435420921092e-2_f64 * t7742 * t9325 + t7712 - t7715;
    (t9315, t9319, t9320, t9324, t9328)
}
