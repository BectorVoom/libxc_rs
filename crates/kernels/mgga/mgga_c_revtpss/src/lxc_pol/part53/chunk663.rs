//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 663/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk663<F: Float>(t7030: F, t807: F, t1945: F, t786: F, t817: F, t64: F, t822: F) -> (F, F, F, F) {
    let t7031 = t807 * t7030;
    let t7032 = F::new(0.14291339372689912324e-4) * t7031;
    let t7033 = t786 * t1945;
    let t7034 = t7033 * t817;
    let t7035 = F::new(0.25410001404642664113e-4) * t7034;
    let t7036 = t822 * t64;
    (t7032, t7033, t7035, t7036)
}
