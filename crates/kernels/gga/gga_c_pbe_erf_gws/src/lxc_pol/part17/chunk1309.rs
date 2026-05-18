//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1309/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1309<F: Float>(t4028: F, t9135: F, t14015: F, t9655: F, t51421: F, t9490: F, t14011: F, t9588: F, t14498: F, t9353: F, t51256: F, t54158: F, t54160: F, t54162: F, t54164: F, t54167: F, t54168: F) -> F {
    let t54170 = t4028 * t9135;
    let t54173 = t14015 * t9655;
    let t54175 = t51421 * t9490;
    let t54177 = t14011 * t9588;
    let t54179 = t14498 * t9353;
    let t54181 = -t54158 / F::new(48.0) - t54160 / F::new(24.0) - t54162 / F::new(192.0) + t54164 / F::new(96.0) + t54167 + t54168 / F::new(24.0) + t54170 / F::new(48.0) + F::new(7.0) / F::new(144.0) * t51256 - t54173 / F::new(96.0) + F::new(5.0) / F::new(192.0) * t54175 + t54177 / F::new(96.0) - t54179 / F::new(64.0);
    t54181
}
