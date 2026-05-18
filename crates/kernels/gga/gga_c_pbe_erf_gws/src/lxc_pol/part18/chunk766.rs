//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 766/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk766<F: Float>(t1641: F, t50: F, t188: F, t9: F, t191: F, t784: F, t190: F, t212: F, t1251: F, t658: F, t205: F, t626: F) -> (F, F, F, F, F, F) {
    let t5002 = F::new(1.0) / t1641 / t50;
    let t5018 = t9 * t188;
    let t5044 = t784 * t191;
    let t5047 = F::new(0.29629629629629629629e-1) * t190 * t5044 * t212;
    let t5052 = t1251 * t658;
    let t5060 = F::new(1.0) / t205 / t626;
    (t5002, t5018, t5044, t5047, t5052, t5060)
}
