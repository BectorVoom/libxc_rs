//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1180/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1180<F: Float>(t265: F, t393: F, t207: F, t7086: F, t1940: F, t1963: F, t198: F, t2403: F, t7091: F, t775: F, t890: F, t892: F, t1100: F, t1102: F, t336: F, t5023: F, t7177: F, t7181: F) -> (F, F) {
    let t394 = t265 < t393;
    let t7188 = t207 * t7086;
    let t7193 = -t1940 * t7091 * t890 + F::new(3.0) * t1963 * t2403 * t775 + t198 * t7188 * t892;
    let t7194 = piecewise3::<F>(t394, t1102 * t198 * t336 * t7177 - t1100 * t5023 * t7181, t7193);
    (t7193, t7194)
}
