//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 595/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk595<F: Float>(t1165: F, t540: F, t945: F, t1530: F, t3371: F, t1535: F, t1533: F, t4289: F, t1162: F, t4180: F, t1180: F, t1531: F, t3172: F, t3179: F, t3462: F, t418: F, t4331: F, t4335: F, t4339: F, t4340: F, t4344: F, t4350: F, t4355: F, t4361: F, t4364: F, t4368: F, t4369: F, t4373: F, t4376: F, t4380: F) -> (F, F, F) {
    let t4384 = t1165 * t540 * t945;
    let t4389 = t1530 * t3371;
    let t4391 = F::cast_from(0.40015750243531754508e-2_f64) * t4389 * t1535;
    let t4393 = t1165 * t4289 * t1533;
    let t4396 = t4180 * t1162;
    let t4398 = F::cast_from(0.85748036236139473944e-3_f64) * t4396 * t1535;
    let t4399 = -F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t4331 + F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t4335 - t4339 + F::cast_from(0.20007875121765877254e-2_f64) * t4340 - F::cast_from(0.51448821741683684367e-2_f64) * t418 * t4344 + F::cast_from(0.85748036236139473944e-3_f64) * t4350 - F::cast_from(0.25724410870841842183e-1_f64) * t418 * t4355 - F::cast_from(0.17149607247227894789e-2_f64) * t4361 + F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t4364 + t4368 + F::new(35.0) / F::new(432.0) * t4369 + F::cast_from(0.21437009059034868486e-3_f64) * t4373 - F::cast_from(0.17149607247227894789e-2_f64) * t3462 * t4376 + F::cast_from(0.85748036236139473944e-3_f64) * t1531 * t4380 - F::cast_from(0.85748036236139473944e-3_f64) * t1531 * t4384 + F::cast_from(0.85748036236139473944e-3_f64) * t3172 - F::cast_from(0.85748036236139473944e-3_f64) * t3179 - t4391 + F::cast_from(0.85748036236139473944e-3_f64) * t1531 * t4393 + t4398;
    (t4384, t4393, t4399)
}
