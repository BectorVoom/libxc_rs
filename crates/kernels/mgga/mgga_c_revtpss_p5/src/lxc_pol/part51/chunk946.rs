//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 946/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk946<F: Float>(t31956: F, t32025: F, t3336: F, t8527: F, t11108: F, t8531: F, t1940: F, t198: F, t207: F, t2403: F, t31858: F, t31863: F, t31876: F, t7086: F, t7091: F, t775: F, t8490: F, t8494: F, t890: F, t892: F) -> (F, F, F, F) {
    let t32026 = t31956 + t32025;
    let t32030 = t8527 * t3336;
    let t32036 = t8531 * t11108;
    let t32058 = t198 * t207 * t31858 * t892 - t1940 * t31863 * t890 + F::cast_from(2.0_f64) * t1940 * t31876 * t890 - F::cast_from(2.0_f64) * t1940 * t7086 * t7091 + F::cast_from(3.0_f64) * t2403 * t775 * t8490 - F::cast_from(3.0_f64) * t2403 * t775 * t8494;
    (t32026, t32030, t32036, t32058)
}
