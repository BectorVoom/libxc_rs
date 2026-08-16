//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 683/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk683<F: Float>(t1989: F, t3336: F, t207: F, t7086: F, t1940: F, t1963: F, t198: F, t2403: F, t7091: F, t775: F, t890: F, t892: F) -> (F, F) {
    let t7181 = t1989 * t3336;
    let t7188 = t207 * t7086;
    let t7193 = -t1940 * t7091 * t890 + F::cast_from(3.0_f64) * t1963 * t2403 * t775 + t198 * t7188 * t892;
    (t7181, t7193)
}
