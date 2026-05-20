//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1518/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1518<F: Float>(t213: F, t23359: F, t262: F, t5966: F, t23148: F, t23421: F, t2411: F, t11064: F, t23429: F, t892: F, t23478: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t77316 = t213 * t23359;
    let t77333 = t5966 * t262;
    let t77341 = t262 * t23148;
    let t77357 = t23421 * t2411;
    let t77373 = t23429 * t11064;
    let t77460 = t23421 * t892;
    let t77499 = t689 * t23478;
    (t77316, t77333, t77341, t77357, t77373, t77460, t77499)
}
