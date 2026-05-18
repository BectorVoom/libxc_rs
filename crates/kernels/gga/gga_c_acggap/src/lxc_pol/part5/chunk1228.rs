//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1228/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1228<F: Float>(t1101: F, t1165: F, t17888: F, t1884: F, t1008: F, t5539: F, t322: F, t368: F, t384: F, t398: F, t5784: F, t1089: F, t1095: F, t13545: F, t17281: F, t17291: F, t17302: F, t17304: F, t17306: F, t1839: F, t301: F, t3201: F, t397: F, t418: F, t5674: F, t6074: F, t966: F) -> F {
    let t22470 = t17888 * t1165 * t1884 * t1101;
    let t22473 = t1008 * t5539;
    let t22488 = t384 * t398 * t368 * t5784 * t322;
    let t22492 = F::new(0.17149607247227894789e-2) * t13545 + F::new(0.85748036236139473944e-3) * t17281 - F::new(0.42874018118069736972e-3) * t397 * t398 * t966 * t1839 + F::new(0.51448821741683684366e-1) * t22470 + F::new(0.17149607247227894789e-2) * t17291 + F::new(0.51448821741683684368e-2) * t22473 + F::new(0.34299214494455789578e-2) * t418 * t1089 * t1095 * t5674 * t301 - F::new(0.90702367218671976884e-1) * t17302 - F::new(0.17149607247227894789e-2) * t418 * t398 * t3201 * t6074 - F::new(0.85748036236139473944e-3) * t22488 - F::new(0.34299214494455789578e-2) * t17304 - F::new(0.34299214494455789577e-2) * t17306;
    t22492
}
