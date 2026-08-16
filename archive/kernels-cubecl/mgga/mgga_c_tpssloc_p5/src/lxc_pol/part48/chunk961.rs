//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 961/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk961<F: Float>(t1880: F, t23218: F, t31366: F, t112759: F, t112761: F, t114808: F, t114811: F, t114815: F, t114822: F, t114827: F, t23278: F, t24297: F, t24330: F, t259: F, t2591: F, t2597: F, t31311: F, t31343: F, t31400: F, t31416: F, t6627: F, t6632: F, t7107: F, t8543: F, t8563: F, t866: F, t87755: F, t9590: F) -> F {
    let t114836 = t1880 * t31366 * t23218;
    let t114838 = -F::cast_from(2.0_f64) * t23278 * t7107 + t2591 * t8543 * t259 - F::cast_from(0.16449340668482264365e-1_f64) * t114808 - t9590 * t8563 + t112759 - t112761 - F::cast_from(2.0_f64) * t114811 * t866 - t114815 + F::cast_from(4.0_f64) * t2597 * t31343 - F::cast_from(12.0_f64) * t87755 * t31416 - F::cast_from(0.49348022005446793095e-1_f64) * t114822 + F::cast_from(4.0_f64) * t24297 * t6632 - F::cast_from(0.82246703342411321824e-2_f64) * t114827 + F::cast_from(2.0_f64) * t6627 * t24330 + F::cast_from(4.0_f64) * t2597 * t31311 - F::cast_from(2.0_f64) * t2597 * t31400 - F::cast_from(0.82246703342411321825e-2_f64) * t114836;
    t114838
}
