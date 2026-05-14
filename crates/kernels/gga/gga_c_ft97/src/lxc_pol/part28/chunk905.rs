//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 905/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk905<F: Float>(t3103: F, t7211: F, t1317: F, t1800: F, t28: F, t34482: F, t469: F, t473: F, t5665: F, t32057: F, t32063: F, t34371: F, t144991: F, t8270: F, t144809: F, t446: F, t7824: F) -> (F, F, F, F, F, F) {
    let t145035 = t7211 * t3103;
    let t145038 = t1317 * t28 * t1800 * t145035;
    let t145042 = t5665 * t28 * t469 * t34482 * t473;
    let t145045 = t32057 * t32063 * t34371;
    let t145048 = t1317 * t28 * t8270 * t144991;
    let t145051 = t446 * t7824 * t144809;
    (t145035, t145038, t145042, t145045, t145048, t145051)
}
