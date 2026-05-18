//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1153/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1153<F: Float>(t13792: F, t14617: F, t14479: F, t14579: F, t14585: F, t14589: F, t14593: F, t14597: F, t14599: F, t14603: F, t14605: F, t14609: F, t14611: F, t14615: F, t335: F) -> F {
    let t14618 = t13792 * t14617;
    let t14620 = t14479 / F::new(96.0) - t335 * t14579 / F::new(96.0) - t14585 / F::new(1536.0) - t14589 / F::new(1536.0) - t14593 / F::new(384.0) - t14597 / F::new(1536.0) - F::new(7.0) / F::new(144.0) * t14599 + t14603 / F::new(768.0) + F::new(7.0) / F::new(4608.0) * t14605 - t14609 / F::new(3072.0) + F::new(7.0) / F::new(4608.0) * t14611 - t14615 / F::new(768.0) - t14618 / F::new(96.0);
    t14620
}
