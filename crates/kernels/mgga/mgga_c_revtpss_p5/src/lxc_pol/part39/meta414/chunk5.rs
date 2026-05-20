//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1497/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1497<F: Float>(t116913: F, t116915: F, t116917: F, t116927: F, t116930: F, t116932: F, t116934: F, t116936: F, t116968: F, t116969: F, t116971: F, t116995: F) -> F {
    let t117572 = F::new(2.0) * t116913 + F::new(20.0) / F::new(9.0) * t116915 + F::new(10.0) / F::new(27.0) * t116917 + F::new(44.0) / F::new(9.0) * t116927 - F::new(110.0) / F::new(27.0) * t116930 - F::new(2.0) / F::new(3.0) * t116932 - F::new(50.0) / F::new(27.0) * t116934 + F::new(5.0) / F::new(9.0) * t116936 + t116968 + F::new(110.0) / F::new(27.0) * t116969 + F::new(40.0) / F::new(27.0) * t116971 - F::new(20.0) / F::new(9.0) * t116995;
    t117572
}
