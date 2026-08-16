//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 491/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk491<F: Float>(t446: F, t5672: F, t6199: F, t1392: F, t1515: F, t1516: F, t1430: F, t1475: F, t221: F, t209: F, t589: F, t1501: F) -> (F, F, F, F) {
    let t6201 = t5672 * t6199 * t446;
    let t6205 = t1515 * t1516 * t1392;
    let t6210 = t221 * t1475 * t1430;
    let t6213 = t209 * t589;
    let t6215 = t221 * t1501 * t6213;
    (t6201, t6205, t6210, t6215)
}
