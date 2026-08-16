//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1366/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1366<F: Float>(t26960: F, t28096: F, t2850: F, t3515: F, t93006: F, t93008: F, t93010: F, t93012: F, t93014: F, t93016: F, t96061: F, t96068: F, t96071: F, t96116: F, t96910: F) -> F {
    let t97232 = F::cast_from(0.23214722222222222222e-2_f64) * t93006 + F::cast_from(0.61905925925925925926e-2_f64) * t93008 - F::cast_from(0.23214722222222222222e-2_f64) * t93010 + F::cast_from(0.15476481481481481481e-2_f64) * t93012 - F::cast_from(0.61905925925925925925e-2_f64) * t96061 - F::cast_from(0.41270617283950617284e-2_f64) * t96068 + F::cast_from(0.92858888888888888886e-2_f64) * t96071 - F::cast_from(0.11326774691358024691e-2_f64) * t93014 - F::cast_from(0.41188271604938271606e-3_f64) * t93016 + F::cast_from(0.13901041666666666667e-2_f64) * t26960 * t96910 - F::cast_from(0.23168402777777777778e-3_f64) * t26960 * t3515 * t28096 * t2850 - F::cast_from(0.15476481481481481481e-2_f64) * t96116;
    t97232
}
