//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 905/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk905(t11222: f64, t200: f64, t1954: f64, t1006: f64, t128: f64, t4864: f64, t11202: f64, t8291: f64, t3640: f64, t518: f64, t11183: f64, t11186: f64, t11190: f64, t11193: f64, t11196: f64, t11200: f64, t11205: f64, t11212: f64, t11218: f64, t11220: f64) -> (f64, f64, f64, f64, f64) {
    let t11223 = t11222 * t200;
    let t11224 = t11223 * t1954;
    let t11225 = t1006 * t11224;
    let t11227 = t4864 * t128;
    let t11228 = t11202 * t11227;
    let t11229 = t11228 * t8291;
    let t11231 = t518 * t3640;
    let t11233 = 0.27155700879230501195e-5_f64 * t11183 + 0.27155700879230501195e-5_f64 * t11186 - 0.60736713313768998074e-4_f64 * t11190 - 0.60736713313768998074e-4_f64 * t11193 - 0.20245571104589666025e-4_f64 * t11196 + 0.43449121406768801912e-5_f64 * t11200 + 0.12653481940368541265e-5_f64 * t11205 - 0.90519002930768337316e-7_f64 * t11212 + 0.11880619134663344273e-5_f64 * t11218 - 0.43449121406768801912e-4_f64 * t11220 - 0.43449121406768801912e-4_f64 * t11225 + 0.12653481940368541265e-5_f64 * t11229 + 0.17379648562707520765e-3_f64 * t11231;
    (t11223, t11224, t11227, t11228, t11233)
}
