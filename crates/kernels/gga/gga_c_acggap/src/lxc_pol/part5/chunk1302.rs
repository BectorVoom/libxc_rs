//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1302/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1302<F: Float>(t1165: F, t1173: F, t1180: F, t1181: F, t1188: F, t18743: F, t18747: F, t18763: F, t18765: F, t21342: F, t24173: F, t24175: F, t24184: F, t24194: F, t24196: F, t24201: F, t301: F, t335: F, t336: F, t4437: F, t4680: F, t530: F, t5867: F, t6395: F) -> F {
    let t24204 = -F::new(0.10289764348336736873e-1) * t18743 - F::new(0.68598428988911579156e-2) * t18747 - F::new(0.17149607247227894789e-2) * t24173 - F::new(0.32012600194825403606e-1) * t24175 + F::new(0.34299214494455789578e-2) * t1173 * t1181 * t530 * t21342 + F::new(0.34299214494455789578e-2) * t1180 * t4680 * t6395 + F::new(0.85748036236139473944e-3) * t1180 * t1165 * t24184 * t1188 + F::new(0.42874018118069736972e-3) * t1180 * t1165 * t5867 * t4437 - F::new(0.34299214494455789578e-2) * t18763 - F::new(0.40015750243531754508e-2) * t24194 - t335 * t336 * t24196 * t301 / F::new(24.0) - F::new(0.17149607247227894789e-2) * t24201 - F::new(0.42874018118069736972e-3) * t18765;
    t24204
}
