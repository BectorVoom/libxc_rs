//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2898/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2898<F: Float>(t52037: F, t52013: F, t52016: F, t52020: F, t52023: F, t52025: F, t52028: F, t52031: F, t52033: F, t52039: F, t52041: F, t52597: F) -> F {
    let t52598 = F::cast_from(0.45908888888888888888e0_f64) * t52037;
    let t52601 = -F::cast_from(0.13892666666666666667e0_f64) * t52013 + F::cast_from(0.62517e0_f64) * t52016 - F::cast_from(0.187551e1_f64) * t52020 + F::cast_from(0.794188125e1_f64) * t52023 - F::cast_from(0.473371875e0_f64) * t52025 + F::cast_from(0.61977000000000000001e1_f64) * t52028 + F::cast_from(0.68863333333333333334e1_f64) * t52031 + F::cast_from(0.309885e1_f64) * t52033 + t52597 - t52598 - F::cast_from(0.20659e1_f64) * t52039 - F::cast_from(0.103295e1_f64) * t52041;
    t52601
}
