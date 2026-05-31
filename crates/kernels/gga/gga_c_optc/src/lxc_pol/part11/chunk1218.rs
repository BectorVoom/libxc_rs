//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1218/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1218<F: Float>(t28552: F, t37467: F, t37470: F, t28559: F, t28561: F, t16287: F, t1879: F, t2198: F, t22296: F, t3593: F, t4611: F, t4744: F, t48045: F, t48051: F, t55893: F, t95: F) -> (F, F, F, F, F, F) {
    let t56012 = F::cast_from(384.0_f64) * t28552;
    let t56013 = F::cast_from(48.0_f64) * t37467;
    let t56014 = F::cast_from(6.0_f64) * t37470;
    let t56015 = F::cast_from(144.0_f64) * t28559;
    let t56016 = F::cast_from(48.0_f64) * t28561;
    let t56024 = F::cast_from(0.46520786582826174894e-1_f64) * t95 * t2198 * t55893 + t56012 - t56013 + t56014 + t56015 - t56016 + t22296 + F::cast_from(6.0_f64) * t48045 + F::cast_from(6.0_f64) * t48051 + F::cast_from(0.31013857721884116596e-1_f64) * t1879 * t3593 * t16287 + F::cast_from(3.0_f64) * t4611 * t4744;
    (t56012, t56013, t56014, t56015, t56016, t56024)
}
