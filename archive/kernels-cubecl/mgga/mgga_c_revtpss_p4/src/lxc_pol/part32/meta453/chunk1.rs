//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1649/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1649<F: Float>(t25234: F, t2677: F, t1941: F, t243: F, t2712: F, t64: F) -> (F, F, F) {
    let t25235 = t25234 * t2677;
    let t25236 = F::cast_from(0.2032800112371413129e-3_f64) * t25235;
    let t25237 = t1941 * t243;
    let t25240 = t64 * t2712;
    (t25236, t25237, t25240)
}
