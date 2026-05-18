//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1355/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1355<F: Float>(t11020: F, t26955: F, t26960: F, t26977: F, t28096: F, t28204: F, t2829: F, t2845: F, t3515: F, t95884: F, t95887: F, t96739: F, t96857: F, t96977: F, t96980: F, t96993: F, t96995: F, t96999: F) -> F {
    let t97006 = -F::new(0.92754700520833333333e-4) * t26955 * t96739 + t96977 - t96980 - F::new(0.46377350260416666666e-4) * t26955 * t96857 + F::new(0.11584201388888888889e-3) * t26960 * t3515 * t28096 * t2829 + F::new(0.15445601851851851852e-3) * t26960 * t11020 * t28096 * t2845 + t96993 + F::new(0.11584201388888888889e-3) * t26960 * t96995 + F::new(0.15445601851851851852e-3) * t26960 * t96999 - F::new(0.11607361111111111111e-2) * t95884 + F::new(0.61905925925925925925e-2) * t95887 - F::new(0.13913205078125e-3) * t28204 * t26977;
    t97006
}
