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
    let t14620 = t14479 / F::cast_from(96.0_f64) - t335 * t14579 / F::cast_from(96.0_f64) - t14585 / F::cast_from(1536.0_f64) - t14589 / F::cast_from(1536.0_f64) - t14593 / F::cast_from(384.0_f64) - t14597 / F::cast_from(1536.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t14599 + t14603 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t14605 - t14609 / F::cast_from(3072.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t14611 - t14615 / F::cast_from(768.0_f64) - t14618 / F::cast_from(96.0_f64);
    t14620
}
