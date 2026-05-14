//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 658/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk658<F: Float>(t1983: F, t407: F, t7586: F, t7585: F, t130: F, t413: F, t577: F) -> (F, F, F, F, F) {
    let t7587 = t1983 * t407;
    let t7588 = t7586 * t7587;
    let t7589 = t7585 * t7588;
    let t7599 = t130 * t413;
    let t7600 = t7599 * t577;
    (t7587, t7588, t7589, t7599, t7600)
}
