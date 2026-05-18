//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1231/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1231<F: Float>(t1165: F, t13299: F, t13573: F, t1532: F, t17139: F, t17314: F, t17316: F, t17318: F, t17327: F, t22538: F, t22540: F, t22544: F, t22546: F, t22550: F, t301: F, t3462: F, t4257: F, t525: F, t6263: F) -> F {
    let t22552 = -F::new(0.10289764348336736873e-1) * t17314 + F::new(0.34299214494455789578e-1) * t17139 * t13299 * t525 * t4257 - F::new(0.42874018118069736972e-3) * t17316 + F::new(0.24009450146119052704e-1) * t17318 - F::new(0.68598428988911579156e-2) * t3462 * t1165 * t1532 * t6263 * t301 + F::new(0.11337795902333997111e-1) * t22538 - F::new(0.40015750243531754508e-2) * t22540 + F::new(0.34299214494455789578e-2) * t17327 - F::new(0.25724410870841842183e-2) * t13573 - F::new(0.40015750243531754508e-1) * t22544 + F::new(0.80031500487063509015e-2) * t22546 + F::new(0.85748036236139473944e-3) * t22550;
    t22552
}
