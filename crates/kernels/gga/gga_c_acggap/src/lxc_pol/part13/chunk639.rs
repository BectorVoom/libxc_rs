//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 639/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk639<F: Float>(t336: F, t7401: F, t7400: F, t2020: F, t374: F, t1145: F, t2041: F, t1117: F, t1121: F, t570: F, t1530: F, t2067: F) -> (F, F, F, F, F, F, F, F) {
    let t7402 = t336 * t7401;
    let t7403 = t7400 * t7402;
    let t7405 = t2020 * t374;
    let t7406 = 7.0 / 144.0 * t7405;
    let t7407 = t2041 * t1145;
    let t7409 = t2041 * t1117;
    let t7411 = t570 * t1121;
    let t7413 = t1530 * t2067;
    (t7402, t7403, t7405, t7406, t7407, t7409, t7411, t7413)
}
