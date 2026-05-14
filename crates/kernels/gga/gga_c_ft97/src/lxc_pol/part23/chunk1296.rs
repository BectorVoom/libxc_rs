//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1296/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1296<F: Float>(t107836: F, t108335: F, t109700: F, t109711: F, t122534: F, t124076: F, t124971: F, t125030: F, t1403: F, t193: F, t2354: F, t24240: F, t28467: F, t30915: F, t3683: F, t3827: F, t4969: F, t5996: F, t6002: F, t6009: F, t6745: F, t6749: F, t6752: F, t6945: F) -> (F,) {
    let t125205 = -t107836 * t6749 / 9.0 - 2.0 / 3.0 * t5996 * t30915 - 2.0 / 3.0 * t1403 * t193 * t108335 * t6752 - 2.0 * t124971 - t1403 * t193 * t124076 * t6009 / 3.0 - 2.0 / 3.0 * t6745 * t28467 - 2.0 * t125030 - 2.0 * t3827 * t6945 - 4.0 * t122534 + t109700 + 4.0 / 27.0 * t109711 - 2.0 * t3683 * t6945 + t6002 * t2354 * t24240 * t4969 / 9.0;
    (t125205,)
}
