//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2127/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2127<F: Float>(t25194: F, t7898: F, t25851: F, t7732: F, t10416: F, t7735: F, t13435: F, t2322: F, t27137: F, t25856: F, t4248: F, t2014: F, t2034: F, t49564: F) -> (F, F, F, F, F, F, F) {
    let t98603 = F::cast_from(2.0_f64) * t7898 * t25194;
    let t98605 = F::cast_from(2.0_f64) * t7732 * t25851;
    let t98607 = F::cast_from(2.0_f64) * t10416 * t7735;
    let t98609 = F::cast_from(4.0_f64) * t13435 * t7735;
    let t98611 = F::cast_from(4.0_f64) * t2322 * t27137;
    let t98615 = F::cast_from(2.0_f64) * t4248 * t25856;
    let t98617 = t2014 * t2034 * t49564;
    (t98603, t98605, t98607, t98609, t98611, t98615, t98617)
}
