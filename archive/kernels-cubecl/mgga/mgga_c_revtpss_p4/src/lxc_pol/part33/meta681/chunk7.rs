//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2228/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2228<F: Float>(t109087: F, t109090: F, t109092: F, t109095: F, t109099: F, t109103: F, t109107: F, t109110: F, t109112: F, t109117: F, t109121: F, t109124: F, t109126: F, t109128: F, t1502: F, t2127: F, t2165: F, t21658: F, t22506: F, t29337: F, t4246: F, t6765: F, t7584: F, t8233: F) -> F {
    let t111770 = -F::cast_from(2.0_f64) * t1502 * t29337 - t2127 * t21658 + t2165 * t22506 - F::cast_from(2.0_f64) * t4246 * t8233 - t6765 * t7584 + t109087 + t109090 - t109092 - t109095 - t109099 + t109103 - t109107 + t109110 + t109112 - t109117 + t109121 + t109124 - t109126 - t109128;
    t111770
}
