//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 687/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk687<F: Float>(t1767: F, t8345: F, t91: F, t965: F, t1766: F, t3157: F, t473: F, t11416: F, t11395: F, t11399: F, t11404: F, t11408: F, t11413: F, t11781: F, t11783: F, t8455: F) -> (F, F, F) {
    let t11787 = t91 * t8345 * t965 * t1767;
    let t11789 = t1766 * t3157;
    let t11791 = t91 * t11789 * t473;
    let t11798 = 4.0 / 9.0 * t11416;
    let t11799 = -t11781 - t8455 - t11783 / 12.0 + t11787 / 8.0 - t11791 / 6.0 - t11395 / 3.0 - 4.0 / 9.0 * t11399 + 22.0 / 27.0 * t11404 + 2.0 / 3.0 * t11408 + 4.0 / 3.0 * t11413 - t11798;
    (t11787, t11791, t11799)
}
