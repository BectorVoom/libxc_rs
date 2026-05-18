//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1202/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1202<F: Float>(t1173: F, t1181: F, t13308: F, t13310: F, t13314: F, t13317: F, t13320: F, t13330: F, t13332: F, t13337: F, t1532: F, t16814: F, t16818: F, t360: F, t4289: F, t5710: F, t6258: F) -> F {
    let t21894 = F::new(0.42874018118069736972e-3) * t13308 + F::new(0.11337795902333997111e-1) * t13310 - t13314 + t13317 - t13320 + t13330 - F::new(0.40015750243531754508e-2) * t13332 + t13337 + F::new(0.17149607247227894789e-2) * t16814 + F::new(0.85748036236139473944e-3) * t16818 + F::new(0.68598428988911579156e-2) * t1173 * t1181 * t4289 * t5710 + F::new(0.68598428988911579156e-2) * t1173 * t1181 * t1532 * t6258 * t360;
    t21894
}
