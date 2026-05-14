//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 944/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk944<F: Float>(t6417: F, t6456: F, t9011: F, t9015: F, t9018: F, t9019: F, t9021: F, t9023: F, t9025: F, t9030: F, t9031: F, t9032: F, t3224: F, t6402: F, t2307: F, t3252: F) -> (F, F, F) {
    let t9536 = -t9011 - t9015 + t9018 - t9019 - t9021 + 7.0 / 2304.0 * t6417 - t9023 - t9025 - t9030 + t9031 + t9032 - 119.0 / 3456.0 * t6456;
    let t9539 = 7.0 / 576.0 * t6402 * t3224;
    let t9540 = t3252 * t2307;
    (t9536, t9539, t9540)
}
