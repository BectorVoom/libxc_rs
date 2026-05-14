//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1188/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1188<F: Float>(t30895: F, t92: F, t10157: F, t107765: F, t107871: F, t1091: F, t109536: F, t1173: F, t1403: F, t18622: F, t193: F, t2347: F, t2354: F, t2360: F, t24204: F, t24231: F, t27939: F, t28030: F, t28036: F, t30883: F, t31320: F, t3875: F, t3886: F, t5053: F, t6002: F, t6003: F, t6005: F, t6008: F, t6745: F, t719: F, t771: F, t96397: F, t96400: F, t96770: F) -> (F,) {
    let t121770 = t30895 * t92;
    let t121807 = -t121770 * t6005 / 18.0 + 2.0 * t6002 * t10157 * t6003 * t18622 + 2.0 / 9.0 * t6002 * t24231 * t107765 * t3875 + 2.0 / 9.0 * t6002 * t28030 * t1173 * t2360 * t3886 - 2.0 / 27.0 * t6002 * t28036 * t1173 * t2347 * t3886 - t1403 * t193 * t6008 * t771 * t5053 / 3.0 - t107871 + 2.0 / 27.0 * t96397 + 2.0 / 27.0 * t96400 - t719 * t31320 - t24204 * t30883 / 9.0 - t6002 * t2354 * t109536 * t1091 / 9.0 - t96770 + t6745 * t27939 / 3.0;
    (t121807,)
}
