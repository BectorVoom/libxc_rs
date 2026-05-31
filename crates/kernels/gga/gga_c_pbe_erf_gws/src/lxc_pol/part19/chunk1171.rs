//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1171/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1171<F: Float>(t14558: F, t14563: F, t14060: F, t14081: F, t14229: F, t14233: F, t14556: F, t14560: F, t14568: F, t14571: F, t15072: F, t15049: F, t15060: F, t15071: F) -> F {
    let t15074 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t14558;
    let t15076 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t14563;
    let t15079 = t15072 - t14556 / F::cast_from(192.0_f64) + t15074 - t14560 / F::cast_from(96.0_f64) + t14060 + t15076 + t14568 / F::cast_from(48.0_f64) - t14571 / F::cast_from(48.0_f64) + t14229 + t14081 + t14233;
    let t15081 = t15049 + t15060 + t15071 + t15079;
    t15081
}
