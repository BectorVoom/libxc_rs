//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 284/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk284<F: Float>(t310: F, t547: F, t315: F, t545: F, t323: F, t145: F, t495: F, t301: F) -> (F, F, F, F, F) {
    let t1306 = t310 * t547;
    let t1308 = t315 * t545;
    let t1309 = t1308 * t323;
    let t1313 = t145 * t495;
    let t1314 = t1313 * t301;
    (t1306, t1308, t1309, t1313, t1314)
}
