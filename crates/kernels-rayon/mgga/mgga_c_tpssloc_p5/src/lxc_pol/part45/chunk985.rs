//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 985/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk985(t1880: f64, t23218: f64, t31366: f64, t112759: f64, t112761: f64, t114808: f64, t114811: f64, t114815: f64, t114822: f64, t114827: f64, t23278: f64, t24297: f64, t24330: f64, t259: f64, t2591: f64, t2597: f64, t31311: f64, t31343: f64, t31400: f64, t31416: f64, t6627: f64, t6632: f64, t7107: f64, t8543: f64, t8563: f64, t866: f64, t87755: f64, t9590: f64) -> f64 {
    let t114836 = t1880 * t31366 * t23218;
    let t114838 = -2.0_f64 * t23278 * t7107 + t2591 * t8543 * t259 - 0.16449340668482264365e-1_f64 * t114808 - t9590 * t8563 + t112759 - t112761 - 2.0_f64 * t114811 * t866 - t114815 + 4.0_f64 * t2597 * t31343 - 12.0_f64 * t87755 * t31416 - 0.49348022005446793095e-1_f64 * t114822 + 4.0_f64 * t24297 * t6632 - 0.82246703342411321824e-2_f64 * t114827 + 2.0_f64 * t6627 * t24330 + 4.0_f64 * t2597 * t31311 - 2.0_f64 * t2597 * t31400 - 0.82246703342411321825e-2_f64 * t114836;
    t114838
}
