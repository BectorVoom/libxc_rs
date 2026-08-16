//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 975/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk975<F: Float>(t2477: F, t4433: F, t828: F, t2672: F, t2686: F, t2704: F, t2742: F, t4345: F, t4350: F, t4355: F, t4357: F, t4359: F, t4362: F, t4368: F, t4373: F, t4426: F, t4431: F, t825: F, t851: F) -> (F, F) {
    let t4435 = t2477 * t828 * t4433;
    let t4439 = -F::cast_from(0.85748036236139473944e-3_f64) * t851 * t4345 - F::cast_from(0.50820002809285328225e-4_f64) * t4350 + F::cast_from(0.71456696863449561619e-5_f64) * t4355 + F::cast_from(0.40015750243531754507e-2_f64) * t4357 + F::cast_from(0.10003937560882938627e-2_f64) * t4359 + F::cast_from(0.42874018118069736972e-3_f64) * t4362 * t4368 - t2672 + t2686 + F::cast_from(0.28582678745379824648e-4_f64) * t4373 + F::cast_from(0.10003937560882938627e-2_f64) * t2742 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t4426 - F::cast_from(0.12705000702321332056e-4_f64) * t4431 + F::cast_from(0.42874018118069736972e-2_f64) * t851 * t4435 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2704;
    (t4435, t4439)
}
