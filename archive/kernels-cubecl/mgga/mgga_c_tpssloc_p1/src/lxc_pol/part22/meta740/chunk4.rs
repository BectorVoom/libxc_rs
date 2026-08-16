//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2441/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2441<F: Float>(t10771: F, t10811: F, t14442: F, t17366: F, t17554: F, t17555: F, t21114: F, t21195: F, t21198: F, t21207: F, t21239: F, t21242: F, t2861: F, t2886: F, t2900: F, t311: F, t41821: F, t42128: F, t42154: F, t42226: F, t42228: F, t4433: F, t4449: F, t49285: F, t49411: F, t5758: F, t5762: F, t5794: F, t68702: F, t69380: F, t69425: F, t69445: F, t931: F, t943: F, t951: F) -> F {
    let t69449 = F::cast_from(0.62071215503128080361e4_f64) * t49285 * t17555 - F::cast_from(0.57895126195293126243e3_f64) * t10771 * t5762 * t4433 - F::cast_from(0.24828486201251232145e5_f64) * t42154 * t21198 * t931 - F::cast_from(2.0_f64) * t2861 * t21195 * t931 + F::cast_from(0.32163958997385070134e2_f64) * t2886 * t69380 * t931 + F::cast_from(0.6207121550312808036e4_f64) * t10811 * t17554 * t4433 + F::cast_from(0.19964560303604640732e6_f64) * t42226 * t21114 * t42228 * t931 + F::cast_from(0.17544670867903938621e1_f64) * t4449 * t17366 + F::cast_from(0.51947577317044391276e2_f64) * t49411 * t5794 - F::cast_from(0.10389515463408878255e3_f64) * t42128 * t21207 + F::cast_from(0.5848223622634646207e0_f64) * t2900 * t21239 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t68702 * t951 + F::cast_from(0.10254018858216406658e4_f64) * t41821 * t21242 + F::cast_from(0.62071215503128080361e4_f64) * t10811 * t5758 * t14442 * t931 - F::cast_from(0.310907e-1_f64) * (t69425 + t69445) * t311;
    t69449
}
