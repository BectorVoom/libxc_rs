//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 126/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk126<F: Float>(t336: F, t339: F, t342: F, t346: F) -> (F, F, F) {
    let t374 = F::new(0.51785e1) * t339 + F::new(0.905775e0) * t336 + F::new(0.1100325e0) * t342 + F::new(0.1241775e0) * t346;
    let t377 = F::new(1.0) + F::cast_from(0.29608749977793437516e2_f64) / t374;
    let t378 = F::ln(t377);
    (t374, t377, t378)
}
