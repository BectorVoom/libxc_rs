//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 899/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk899<F: Float>(t1940: F, t2403: F, t30: F, t31859: F, t31863: F, t31873: F, t31876: F, t605: F, t7010: F, t7091: F, t7092: F, t8490: F, t8494: F, t198: F, t207: F, t31858: F, t7086: F, t775: F, t890: F, t892: F) -> (F, F) {
    let t31882 = 3.0 / 2.0 * t2403 * t8490 * t7010 + t1940 * t31859 * t30 / 2.0 - t1940 * t31863 * t7092 / 2.0 + t1940 * t8490 * t605 / 2.0 - 3.0 / 2.0 * t2403 * t8494 * t7010 - t1940 * t7091 * t31873 + t1940 * t31876 * t7092 - t1940 * t8494 * t605 / 2.0;
    let t32058 = t198 * t207 * t31858 * t892 - t1940 * t31863 * t890 + 2.0 * t1940 * t31876 * t890 - 2.0 * t1940 * t7086 * t7091 + 3.0 * t2403 * t775 * t8490 - 3.0 * t2403 * t775 * t8494;
    (t31882, t32058)
}
