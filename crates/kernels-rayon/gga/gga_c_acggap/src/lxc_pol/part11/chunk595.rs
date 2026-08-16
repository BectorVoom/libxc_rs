//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 595/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk595(t1165: f64, t540: f64, t945: f64, t1530: f64, t3371: f64, t1535: f64, t1533: f64, t4289: f64, t1162: f64, t4180: f64, t1180: f64, t1531: f64, t3172: f64, t3179: f64, t3462: f64, t418: f64, t4331: f64, t4335: f64, t4339: f64, t4340: f64, t4344: f64, t4350: f64, t4355: f64, t4361: f64, t4364: f64, t4368: f64, t4369: f64, t4373: f64, t4376: f64, t4380: f64) -> (f64, f64, f64) {
    let t4384 = t1165 * t540 * t945;
    let t4389 = t1530 * t3371;
    let t4391 = 0.40015750243531754508e-2_f64 * t4389 * t1535;
    let t4393 = t1165 * t4289 * t1533;
    let t4396 = t4180 * t1162;
    let t4398 = 0.85748036236139473944e-3_f64 * t4396 * t1535;
    let t4399 = -0.42874018118069736972e-3_f64 * t1180 * t4331 + 0.42874018118069736972e-3_f64 * t1180 * t4335 - t4339 + 0.20007875121765877254e-2_f64 * t4340 - 0.51448821741683684367e-2_f64 * t418 * t4344 + 0.85748036236139473944e-3_f64 * t4350 - 0.25724410870841842183e-1_f64 * t418 * t4355 - 0.17149607247227894789e-2_f64 * t4361 + 0.85748036236139473944e-3_f64 * t1180 * t4364 + t4368 + 35.0_f64 / 432.0_f64 * t4369 + 0.21437009059034868486e-3_f64 * t4373 - 0.17149607247227894789e-2_f64 * t3462 * t4376 + 0.85748036236139473944e-3_f64 * t1531 * t4380 - 0.85748036236139473944e-3_f64 * t1531 * t4384 + 0.85748036236139473944e-3_f64 * t3172 - 0.85748036236139473944e-3_f64 * t3179 - t4391 + 0.85748036236139473944e-3_f64 * t1531 * t4393 + t4398;
    (t4384, t4393, t4399)
}
