//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 301/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk301<F: Float>(t1140: F, t374: F, t145: F, t360: F, t301: F, t336: F, t167: F, t19: F, t56: F, t124: F) -> (F, F, F, F, F) {
    let t1141 = t1140 * t374;
    let t1143 = t360 * t145;
    let t1145 = t336 * t1143 * t301;
    let t1149 = t56 * t167 * t19;
    let t1150 = t124 * t1149;
    (t1141, t1143, t1145, t1149, t1150)
}
