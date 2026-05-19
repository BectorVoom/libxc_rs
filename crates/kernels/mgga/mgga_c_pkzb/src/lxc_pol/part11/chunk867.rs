//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 867/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk867<F: Float>(t2900: F, t9314: F, t302: F, t5955: F, t759: F, t9282: F, t2901: F, t2104: F, t2887: F, t2899: F, t2922: F, t3647: F, t5984: F, t7664: F, t7712: F, t7715: F, t7736: F, t7742: F, t9270: F, t9274: F, t9279: F, t9284: F, t9289: F, t9293: F, t9298: F, t9302: F, t9308: F, t9311: F) -> (F, F, F, F, F) {
    let t9315 = t2900 * t9314;
    let t9316 = t302 * t9315;
    let t9319 = t5955 * t759;
    let t9320 = t9282 * t9319;
    let t9321 = t302 * t9320;
    let t9324 = t9282 * t2901;
    let t9325 = t302 * t9324;
    let t9328 = -F::cast_from(0.42874018118069736972e-3_f64) * t2104 * t9270 + F::cast_from(0.42874018118069736972e-3_f64) * t2922 * t9274 - F::cast_from(0.42874018118069736972e-3_f64) * t2922 * t9279 + F::cast_from(0.21437009059034868486e-3_f64) * t7664 * t9284 + F::cast_from(0.12862205435420921092e-2_f64) * t2104 * t9289 + F::cast_from(0.85748036236139473944e-3_f64) * t2922 * t9293 - t2887 * t9298 / F::new(16.0) + t2887 * t9302 / F::new(24.0) + F::cast_from(0.45732285992607719437e-2_f64) * t5984 * t3647 - F::cast_from(0.57165357490759649296e-3_f64) * t9308 - F::cast_from(0.85748036236139473944e-3_f64) * t2899 * t9311 + F::cast_from(0.85748036236139473944e-3_f64) * t2899 * t9316 + F::cast_from(0.12862205435420921092e-2_f64) * t7736 * t9321 - F::cast_from(0.12862205435420921092e-2_f64) * t7742 * t9325 + t7712 - t7715;
    (t9315, t9319, t9320, t9324, t9328)
}
