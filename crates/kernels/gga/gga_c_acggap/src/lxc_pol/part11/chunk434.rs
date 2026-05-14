//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 434/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk434<F: Float>(t2068: F, t2074: F, t1988: F, t601: F, t381: F, t597: F) -> (F, F, F) {
    let t2075 = t2068 * t2074;
    let t2077 = t1988 * t601;
    let t2078 = 0.10718504529517434243e-3 * t2077;
    let t2079 = t381 * t597;
    (t2075, t2078, t2079)
}
