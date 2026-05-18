//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1376/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1376<F: Float>(t1206: F, t353: F, t3717: F, t4386: F, t14888: F, t26958: F, t15036: F, t14185: F, t3886: F, t859: F, t11375: F, t14911: F, t22379: F, t2376: F, t27047: F, t335: F, t338: F, t3907: F, t4111: F, t55739: F, t55741: F, t55745: F, t57386: F, t57390: F, t57393: F, t57395: F, t58050: F, t6793: F, t814: F, t8654: F) -> F {
    let t58553 = t4386 * t353 * t1206 * t3717;
    let t58556 = t26958 * t14888;
    let t58558 = t26958 * t15036;
    let t58562 = t859 * t353 * t14185 * t3886;
    let t58580 = t22379 * t14888 / F::new(24.0) + t6793 * t58553 / F::new(48.0) - F::new(7.0) / F::new(72.0) * t58556 - F::new(7.0) / F::new(72.0) * t58558 + t6793 * t58562 / F::new(48.0) - t8654 * t14911 / F::new(48.0) + t57386 / F::new(96.0) - t11375 * t27047 * t2376 * t58050 * t814 / F::new(48.0) - t55739 + t55741 - t57390 / F::new(8.0) - t335 * t338 * t3907 * t4111 / F::new(96.0) + t57393 / F::new(12.0) + t55745 + t57395 / F::new(24.0);
    t58580
}
