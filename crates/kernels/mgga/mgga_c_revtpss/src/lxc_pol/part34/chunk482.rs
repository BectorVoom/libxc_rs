//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 482/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk482<F: Float>(t1941: F, t268: F, t404: F, t1263: F, t159: F, t635: F) -> (F, F, F, F, F) {
    let t3356 = t268 * t1941 * t404;
    let t3357 = 0.23744444444444444444e-1 * t3356;
    let t3360 = t159 * t1263;
    let t3361 = t635 * t635;
    let t3362 = 1.0 / t3361;
    (t3356, t3357, t3360, t3361, t3362)
}
