//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1371/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1371<F: Float>(t14682: F, t3140: F, t3989: F, t57321: F, t13815: F, t3781: F, t833: F, t850: F, t11737: F, t1193: F, t14802: F, t2408: F, t2409: F, t26654: F, t3060: F, t3207: F, t3212: F, t35566: F, t4155: F, t53614: F, t54492: F, t54505: F, t54532: F, t57542: F, t57545: F, t57551: F, t57555: F, t57570: F, t9283: F) -> F {
    let t57574 = t3989 * t14682 * t57321 * t3140;
    let t57578 = t850 * t3781 * t13815 * t833;
    let t57580 = t54492 + t2408 * t2409 * t26654 * t4155 / F::new(24.0) + F::new(7.0) / F::new(144.0) * t57542 + t54505 - t57545 / F::new(48.0) - t3207 * t9283 * t1193 * t11737 / F::new(16.0) + t57551 / F::new(48.0) + t57555 / F::new(1536.0) - t2408 * t35566 * t14802 / F::new(12.0) + t54532 - t2408 * t9283 * t53614 * t3212 / F::new(12.0) - t2408 * t9283 * t53614 * t3060 / F::new(12.0) - t57570 / F::new(512.0) + t57574 / F::new(1536.0) + t57578 / F::new(96.0);
    t57580
}
