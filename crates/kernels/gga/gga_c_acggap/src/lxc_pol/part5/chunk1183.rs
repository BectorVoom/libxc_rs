//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1183/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1183<F: Float>(t14223: F, t5749: F, t1017: F, t1165: F, t1173: F, t1180: F, t13156: F, t13161: F, t16637: F, t16639: F, t16641: F, t16644: F, t16646: F, t16648: F, t16663: F, t17544: F, t1759: F, t21128: F, t4267: F, t4680: F, t5611: F) -> F {
    let t21504 = t14223 * t5749;
    let t21513 = -F::new(0.85748036236139473944e-3) * t13156 + F::new(0.85748036236139473944e-3) * t13161 + F::new(0.64025200389650807212e-1) * t16637 + F::new(0.32012600194825403606e-1) * t16639 + F::new(0.32012600194825403606e-1) * t16641 - F::new(0.18140473443734395377e0) * t16644 + F::new(0.16006300097412701803e-1) * t16646 + F::new(0.16006300097412701803e-1) * t16648 - F::new(0.13719685797782315831e-1) * t16663 + F::new(0.10289764348336736873e-1) * t1180 * t1165 * t17544 * t1759 * t1017 + F::new(0.32012600194825403606e-1) * t21504 - F::new(0.34299214494455789578e-2) * t1173 * t1165 * t4267 * t21128 + F::new(0.34299214494455789578e-2) * t1180 * t4680 * t5611;
    t21513
}
