//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2884/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2884<F: Float>(t52035: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52112: F) -> F {
    let t52346 = F::cast_from(0.24722222222222222222e-1_f64) * t52035;
    let t52366 = t52346 - F::cast_from(0.82407407407407407408e-2_f64) * t52037 - F::cast_from(0.37083333333333333333e-1_f64) * t52039 - F::cast_from(0.18541666666666666667e-1_f64) * t52041 - F::cast_from(0.37083333333333333334e-1_f64) * t52045 + F::cast_from(0.12361111111111111111e-1_f64) * t52047 + F::cast_from(0.61805555555555555556e-2_f64) * t52049 + F::cast_from(0.10300925925925925926e-1_f64) * t52051 - F::cast_from(0.18541666666666666666e-1_f64) * t52054 - F::cast_from(0.18541666666666666666e-1_f64) * t52057 - F::cast_from(0.30902777777777777778e-1_f64) * t52060 - F::cast_from(0.166875e0_f64) * t52063 - F::cast_from(0.18541666666666666667e-1_f64) * t41365 + F::cast_from(0.61805555555555555556e-2_f64) * t41367 + F::cast_from(0.18541666666666666667e-1_f64) * t41308 - F::cast_from(0.12361111111111111111e-1_f64) * t41330 - F::cast_from(0.82407407407407407408e-2_f64) * t41332 + F::cast_from(0.30902777777777777778e-2_f64) * t41334 + F::cast_from(0.34336419753086419753e-2_f64) * t41336 - F::cast_from(0.166875e0_f64) * t52112;
    t52366
}
