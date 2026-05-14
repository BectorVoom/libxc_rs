//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1002/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1002<F: Float>(t14365: F, t1940: F, t1963: F, t198: F, t207: F, t2394: F, t2403: F, t2408: F, t2430: F, t25435: F, t25440: F, t25445: F, t2832: F, t4541: F, t7087: F, t7091: F, t775: F, t890: F, t892: F) -> (F,) {
    let t25743 = t198 * t207 * t25435 * t892 - 6.0 * t14365 * t2403 * t7091 + 2.0 * t1940 * t2408 * t25445 - 2.0 * t1940 * t25440 * t890 - t1940 * t2832 * t7091 + 6.0 * t1963 * t2394 * t4541 + 3.0 * t1963 * t2403 * t2430 + 6.0 * t2403 * t7087 * t775;
    (t25743,)
}
