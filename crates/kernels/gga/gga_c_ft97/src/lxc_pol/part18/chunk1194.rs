//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1194/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1194<F: Float>(t100315: F, t22958: F, t5674: F, t24: F, t8270: F, t22952: F, t23050: F, t6469: F, t1636: F, t6520: F, t89: F, t1871: F, t23057: F, t25888: F, t1564: F, t925: F, t93509: F) -> (F, F, F, F, F) {
    let t101631 = t5674 * t22958 * t100315;
    let t101633 = t24 * t8270;
    let t101636 = t22952 * t101633 * t6469 * t23050;
    let t101638 = t89 * t1636 * t6520;
    let t101642 = t22952 * t1871 * t23057 * t25888;
    let t101646 = t5674 * t1564 * t93509 * t925;
    (t101631, t101636, t101638, t101642, t101646)
}
