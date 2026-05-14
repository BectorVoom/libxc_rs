//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1159/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1159<F: Float>(t100417: F, t11397: F, t1901: F, t1564: F, t23057: F, t25878: F, t3052: F, t1317: F, t1637: F, t6508: F, t26027: F, t375: F, t89: F, t1755: F, t446: F, t6469: F, t8411: F) -> (F, F, F, F, F, F) {
    let t100419 = t1901 * t100417 * t11397;
    let t100423 = t25878 * t1564 * t23057 * t3052;
    let t100427 = t1317 * t1637 * t6508;
    let t100430 = t89 * t375 * t26027;
    let t100431 = 2.0 / 9.0 * t100430;
    let t100434 = t446 * t8411 * t6469 * t1755;
    (t100419, t100423, t100427, t100430, t100431, t100434)
}
