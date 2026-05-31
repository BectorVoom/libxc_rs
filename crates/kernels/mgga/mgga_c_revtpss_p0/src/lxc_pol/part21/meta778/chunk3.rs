//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2773/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2773<F: Float>(t10627: F, t10628: F, t10632: F, t14633: F, t14643: F, t14653: F, t14656: F, t14659: F, t1553: F, t18592: F, t231: F, t2634: F, t2642: F, t4409: F, t4415: F, t4417: F, t4420: F, t50396: F, t50914: F, t73: F, t830: F, t833: F) -> F {
    let t50916 = (-F::cast_from(360.0_f64) * t10627 * t4415 * t50396 - F::cast_from(36.0_f64) * t2634 * t4417 * t73 + F::cast_from(60.0_f64) * t10628 * t1553 - F::cast_from(36.0_f64) * t10632 * t18592 + F::cast_from(9.0_f64) * t14633 * t833 - F::cast_from(72.0_f64) * t14643 * t14653 - F::cast_from(36.0_f64) * t14643 * t14656 + F::cast_from(9.0_f64) * t14659 * t830 + F::cast_from(9.0_f64) * t2634 * t4420 + F::cast_from(9.0_f64) * t2642 * t4409 + t50914) * t231;
    t50916
}
