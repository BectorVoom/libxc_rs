//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1172/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1172<F: Float>(t13293: F, t15386: F, t21118: F, t525: F, t3621: F, t6380: F, t6384: F, t1083: F, t1165: F, t1459: F, t1531: F, t16388: F, t16390: F, t16392: F, t16398: F, t336: F, t360: F, t3616: F, t367: F, t372: F, t398: F, t418: F, t4838: F, t5141: F, t535: F, t5674: F, t5867: F, t6374: F, t839: F, t960: F) -> F {
    let t21189 = t13293 * t15386 * t525 * t21118;
    let t21209 = t3621 * t6380;
    let t21211 = t3621 * t6384;
    let t21217 = -F::new(0.17149607247227894789e-2) * t1531 * t1165 * t5867 * t5141 + F::new(0.51448821741683684366e-2) * t21189 - t367 * t336 * t535 * t4838 / F::new(48.0) - F::new(0.12004725073059526352e-1) * t16388 - F::new(0.90702367218671976884e-1) * t16390 - F::new(0.85748036236139473944e-3) * t16392 - F::new(0.22675591804667994222e-1) * t16398 - F::new(0.17149607247227894789e-2) * t418 * t398 * t1083 * t5674 * t360 + F::new(0.25724410870841842184e-2) * t418 * t398 * t1459 * t5674 * t372 + F::new(7.0) / F::new(24.0) * t21209 - F::new(7.0) / F::new(12.0) * t21211 - t3616 * t960 * t6374 * t839 / F::new(4.0);
    t21217
}
