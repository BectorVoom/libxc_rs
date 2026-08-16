//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 593/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk593(t1167: f64, t2411: f64, t824: f64, t758: f64, t179: f64, t3026: f64, t932: f64, t1238: f64, t2377: f64, t2380: f64, t2404: f64, t2408: f64, t3206: f64, t3209: f64, t3214: f64, t3217: f64, t3225: f64, t3230: f64, t3235: f64, t404: f64, t923: f64, t934: f64) -> (f64, f64, f64, f64, f64) {
    let t3236 = t2411 * t1167;
    let t3237 = t3236 * t824;
    let t3238 = t758 * t3237;
    let t3242 = t179 * t932 * t3026;
    let t3245 = -0.21437009059034868486e-3_f64 * t3206 * t3209 - 0.11433071498151929859e-2_f64 * t3214 * t923 - 0.7622047665434619906e-3_f64 * t3217 + 0.22866142996303859718e-2_f64 * t1238 * t934 + 0.14291339372689912324e-3_f64 * t2377 - t2404 - 0.28582678745379824648e-3_f64 * t2408 - 0.42874018118069736972e-3_f64 * t2380 * t3225 - 0.28582678745379824648e-3_f64 * t3230 + 0.12862205435420921092e-2_f64 * t3235 * t3238 - 0.42874018118069736972e-3_f64 * t404 * t3242;
    (t3236, t3237, t3238, t3242, t3245)
}
