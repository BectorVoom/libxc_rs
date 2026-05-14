//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1169/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1169<F: Float>(t2019: F, t2111: F, t17766: F, t7832: F, t2036: F, t2177: F, t91: F, t204: F, t3981: F, t824: F) -> (F, F, F, F, F) {
    let t18304 = t2019 * t2111;
    let t18319 = t7832 * t17766;
    let t18326 = t2036 * t2111;
    let t18406 = t2177 * t2177;
    let t18408 = 1.0 / t91 / t18406;
    let t18427 = t204 * t3981 * t824;
    (t18304, t18319, t18326, t18408, t18427)
}
