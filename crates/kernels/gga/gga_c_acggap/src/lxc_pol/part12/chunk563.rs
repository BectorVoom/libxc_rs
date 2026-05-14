//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 563/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk563<F: Float>(t1535: F, t4396: F, t1180: F, t1531: F, t3172: F, t3179: F, t3462: F, t418: F, t4331: F, t4335: F, t4339: F, t4340: F, t4344: F, t4350: F, t4355: F, t4361: F, t4364: F, t4368: F, t4369: F, t4373: F, t4376: F, t4380: F, t4384: F, t4391: F, t4393: F) -> (F,) {
    let t4398 = 0.85748036236139473944e-3 * t4396 * t1535;
    let t4399 = -0.42874018118069736972e-3 * t1180 * t4331 + 0.42874018118069736972e-3 * t1180 * t4335 - t4339 + 0.20007875121765877254e-2 * t4340 - 0.51448821741683684367e-2 * t418 * t4344 + 0.85748036236139473944e-3 * t4350 - 0.25724410870841842183e-1 * t418 * t4355 - 0.17149607247227894789e-2 * t4361 + 0.85748036236139473944e-3 * t1180 * t4364 + t4368 + 35.0 / 432.0 * t4369 + 0.21437009059034868486e-3 * t4373 - 0.17149607247227894789e-2 * t3462 * t4376 + 0.85748036236139473944e-3 * t1531 * t4380 - 0.85748036236139473944e-3 * t1531 * t4384 + 0.85748036236139473944e-3 * t3172 - 0.85748036236139473944e-3 * t3179 - t4391 + 0.85748036236139473944e-3 * t1531 * t4393 + t4398;
    (t4399,)
}
