//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1115/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1115<F: Float>(t28614: F, t28616: F, t28618: F, t28620: F, t28622: F, t28625: F, t28627: F, t28630: F, t28632: F, t28634: F, t28636: F, t28638: F, t28641: F) -> F {
    let t28697 = F::new(0.9375e-1) * t28614 + F::new(0.625e-1) * t28616 + F::new(0.20234375e-1) * t28618 + F::cast_from(0.10791666666666666667e0_f64) * t28620 - F::cast_from(0.20833333333333333333e-1_f64) * t28622 - F::new(0.4046875e-1) * t28625 + F::cast_from(0.26979166666666666667e-1_f64) * t28627 - F::cast_from(0.89930555555555555557e-2_f64) * t28630 - F::new(0.25e0) * t28632 - F::new(0.9375e-1) * t28634 - F::new(0.625e-1) * t28636 + F::cast_from(0.26979166666666666667e-1_f64) * t28638 + F::new(0.625e-1) * t28641;
    t28697
}
