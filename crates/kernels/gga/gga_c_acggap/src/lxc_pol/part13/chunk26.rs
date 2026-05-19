//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 26/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk26<F: Float>(t11: F, t14: F, t17: F, t25: F) -> (F, F, F, F) {
    let t67 = F::new(0.705945e1) * t14 + F::new(0.1549425e1) * t11 + F::new(0.420775e0) * t17 + F::new(0.1562925e0) * t25;
    let t70 = F::new(1.0) + F::cast_from(0.32163958997385070134e2_f64) / t67;
    let t71 = F::ln(t70);
    let t75 = F::new(1.0) + F::new(0.278125e-1) * t11;
    (t67, t70, t71, t75)
}
