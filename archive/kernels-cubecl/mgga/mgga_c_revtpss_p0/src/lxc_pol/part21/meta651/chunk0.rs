//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2437/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2437<F: Float>(t1032: F, t1040: F, t11902: F, t11762: F, t3241: F, t11752: F, t11755: F, t1011: F, t3247: F, t697: F, t3254: F, t11789: F, t11937: F) -> (F, F, F, F, F, F, F) {
    let t42235 = t11902 * t1032 * t1040;
    let t42240 = t3241 * t11762;
    let t42249 = t3241 * t11752;
    let t42251 = t3241 * t11755;
    let t42254 = t1011 * t697 * t3247;
    let t42257 = t1011 * t697 * t3254;
    let t42259 = t11789 * t11937;
    (t42235, t42240, t42249, t42251, t42254, t42257, t42259)
}
