//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 270/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk270<F: Float>(t45: F, t57: F, t706: F, t707: F, t606: F, t78: F, t81: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t709 = F::new(4.0) * t706 * t707;
    let t712 = piecewise3::<F>(t151, F::new(0.0), F::new(4.0) / F::new(3.0) * t78 * t606);
    let t715 = piecewise3::<F>(t155, F::new(0.0), -F::new(4.0) / F::new(3.0) * t81 * t606);
    let t716 = t712 + t715;
    (t709, t716)
}
