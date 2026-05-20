//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2857/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2857<F: Float>(t52013: F, t52016: F, t52020: F, t52023: F, t52025: F, t52028: F, t52031: F, t52033: F, t52035: F, t52037: F, t52039: F, t52041: F) -> F {
    let t52043 = -F::new(0.11038e0) * t52013 + F::new(0.49671e0) * t52016 - F::new(0.149013e1) * t52020 + F::new(0.58258125e1) * t52023 - F::cast_from(0.1237865625e0_f64) * t52025 + F::cast_from(0.36230999999999999999e1_f64) * t52028 + F::cast_from(0.40256666666666666666e1_f64) * t52031 + F::new(0.181155e1) * t52033 + F::cast_from(0.80513333333333333334e0_f64) * t52035 - F::cast_from(0.26837777777777777778e0_f64) * t52037 - F::new(0.12077e1) * t52039 - F::new(0.60385e0) * t52041;
    t52043
}
