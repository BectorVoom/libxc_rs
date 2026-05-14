//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1304/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1304<F: Float>(t31972: F, t5: F, t18795: F, t18799: F, t18802: F, t18946: F, t18953: F, t19906: F, t19927: F, t25504: F, t29425: F, t29429: F, t4382: F, t4635: F, t5475: F, t6400: F, t6403: F, t911: F, t992: F) -> (F,) {
    let t125427 = t5 * t31972;
    let t125442 = t6403 * t18953 / 4.0 + t29429 * t4382 / 2.0 + t6403 * t18795 / 2.0 + t6403 * t19927 / 4.0 + t6403 * t18799 / 2.0 - t6403 * t18802 + t125427 * t911 / 4.0 + t6403 * t19906 / 4.0 + t6403 * t18946 / 4.0 + t5 * t6400 * t4635 / 4.0 + t5 * t29425 * t992 / 2.0 + t25504 * t5475 / 4.0;
    (t125442,)
}
