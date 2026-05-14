//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 356/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk356<F: Float>(t1204: F, t1210: F, t1215: F, t1271: F, t1274: F, t1295: F, t460: F, t495: F) -> (F,) {
    let t1298 = 0.65854491829355115987e0 * t1204 * t495 - 0.65854491829355115987e0 * t1210 * t1215 + 0.65854491829355115987e0 * t460 * t1271 - 0.65854491829355115987e0 * t1274 * t1295;
    (t1298,)
}
