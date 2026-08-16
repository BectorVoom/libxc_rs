//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 528/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk528(t2477: f64, t4433: f64, t828: f64, t2672: f64, t2686: f64, t2704: f64, t2742: f64, t4345: f64, t4350: f64, t4355: f64, t4357: f64, t4359: f64, t4362: f64, t4368: f64, t4373: f64, t4426: f64, t4431: f64, t825: f64, t851: f64) -> (f64, f64) {
    let t4435 = t2477 * t828 * t4433;
    let t4439 = -0.85748036236139473944e-3_f64 * t851 * t4345 - 0.50820002809285328225e-4_f64 * t4350 + 0.71456696863449561619e-5_f64 * t4355 + 0.40015750243531754507e-2_f64 * t4357 + 0.10003937560882938627e-2_f64 * t4359 + 0.42874018118069736972e-3_f64 * t4362 * t4368 - t2672 + t2686 + 0.28582678745379824648e-4_f64 * t4373 + 0.10003937560882938627e-2_f64 * t2742 - 0.21437009059034868486e-3_f64 * t825 * t4426 - 0.12705000702321332056e-4_f64 * t4431 + 0.42874018118069736972e-2_f64 * t851 * t4435 + 7.0_f64 / 144.0_f64 * t2704;
    (t4435, t4439)
}
