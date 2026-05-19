//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 57/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk57<F: Float>(t12: F, t13: F, t138: F, t145: F) -> (F, F, F, F) {
    let t163 = F::new(0.705945e1) * t13 + F::new(0.1549425e1) * t12 + F::new(0.420775e0) * t138 + F::new(0.1562925e0) * t145;
    let t166 = F::new(1.0) + F::cast_from(0.32163958997385070134e2_f64) / t163;
    let t167 = F::ln(t166);
    let t171 = F::new(1.0) + F::new(0.278125e-1) * t12;
    (t163, t166, t167, t171)
}
