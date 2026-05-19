//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 807/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk807<F: Float>(t1552: F, t6394: F, t1181: F, t1532: F, t5616: F, t1759: F, t322: F, t1165: F, t1163: F, t1150: F, t1180: F, t3616: F, t3816: F, t5253: F, t5263: F, t5288: F, t6376: F, t6380: F, t6384: F, t6389: F) -> (F, F, F, F, F, F, F) {
    let t6395 = t1552 * t6394;
    let t6396 = t1181 * t6395;
    let t6399 = t1532 * t5616;
    let t6400 = t1181 * t6399;
    let t6403 = t1759 * t322;
    let t6405 = t1165 * t1552 * t6403;
    let t6406 = t1163 * t6405;
    let t6408 = -t3616 * t6376 / F::new(4.0) - t1150 * t6380 / F::new(16.0) + t1150 * t6384 / F::new(8.0) + t1150 * t6389 / F::new(16.0) + F::new(35.0) / F::new(432.0) * t3816 + t5253 - F::cast_from(0.16006300097412701803e-1_f64) * t5263 + F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t6396 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t6400 - F::cast_from(0.85748036236139473944e-3_f64) * t6406 - t5288;
    (t6395, t6396, t6399, t6400, t6403, t6405, t6408)
}
