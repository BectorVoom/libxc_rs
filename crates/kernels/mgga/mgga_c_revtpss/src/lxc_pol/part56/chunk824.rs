//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 824/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk824<F: Float>(t1940: F, t198: F, t207: F, t2403: F, t31858: F, t31863: F, t31876: F, t7086: F, t7091: F, t775: F, t8490: F, t8494: F, t890: F, t892: F, t33: F, t1113: F, t31859: F, t7200: F, t7207: F) -> (F, F) {
    let t32058 = t198 * t207 * t31858 * t892 - t1940 * t31863 * t890 + 2.0 * t1940 * t31876 * t890 - 2.0 * t1940 * t7086 * t7091 + 3.0 * t2403 * t775 * t8490 - 3.0 * t2403 * t775 * t8494;
    let t32080 = t33 * t7086;
    let t32088 = 3.0 / 2.0 * t2403 * t8490 * t7200 + t1940 * t31859 * t33 / 2.0 - t1940 * t31863 * t7207 / 2.0 + t1940 * t8490 * t1113 / 2.0 - 3.0 / 2.0 * t2403 * t8494 * t7200 - t1940 * t7091 * t32080 + t1940 * t31876 * t7207 - t1940 * t8494 * t1113 / 2.0;
    (t32058, t32088)
}
