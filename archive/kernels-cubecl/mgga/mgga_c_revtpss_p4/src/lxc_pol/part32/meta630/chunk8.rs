//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2039/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2039<F: Float>(t107882: F, t107885: F, t107895: F, t107939: F, t107943: F, t107947: F, t107985: F, t108028: F, t108036: F, t1940: F, t2071: F, t2403: F, t26425: F, t27773: F, t27777: F, t27810: F, t27817: F, t28460: F, t28472: F, t29949: F, t30420: F, t7200: F, t7428: F, t8020: F) -> F {
    let t110920 = F::cast_from(3.0_f64) * t2403 * t8020 * t27777 + F::cast_from(2.0_f64) * t28472 * t108036 + t28472 * t108028 + F::cast_from(3.0_f64) * t2403 * t8020 * t27773 - F::cast_from(3.0_f64) * t28472 * t107947 - F::cast_from(3.0_f64) * t26425 * t107985 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t107939 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t107943 + F::cast_from(3.0_f64) * t2403 * t7428 * t29949 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t107882 + F::cast_from(3.0_f64) * t2403 * t8020 * t27810 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t30420 * t7200 - F::cast_from(3.0_f64) * t26425 * t107895 - F::cast_from(3.0_f64) * t26425 * t107885 - t1940 * t28460 * t27817;
    t110920
}
