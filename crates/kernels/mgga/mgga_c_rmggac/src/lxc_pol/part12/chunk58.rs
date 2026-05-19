//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 58/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk58<F: Float>(t12: F, t13: F, t138: F, t145: F) -> (F, F, F) {
    let t176 = F::new(0.51785e1) * t13 + F::new(0.905775e0) * t12 + F::new(0.1100325e0) * t138 + F::new(0.1241775e0) * t145;
    let t179 = F::new(1.0) + F::cast_from(0.29608749977793437516e2_f64) / t176;
    let t180 = F::ln(t179);
    (t176, t179, t180)
}
