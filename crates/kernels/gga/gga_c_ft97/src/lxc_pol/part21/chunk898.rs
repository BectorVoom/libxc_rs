//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 898/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk898<F: Float>(t2112: F, t26888: F, t1369: F, t28: F, t3450: F, t5900: F, t9432: F, t5899: F, t23616: F, t23629: F, t23650: F, t27028: F, t27032: F, t27037: F, t27041: F, t27045: F, t27049: F, t27051: F) -> (F, F, F, F, F) {
    let t27053 = t2112 * t26888;
    let t27055 = t1369 * t28 * t27053;
    let t27059 = t9432 * t5900 * t3450;
    let t27060 = t5899 * t27059;
    let t27063 = t27028 / 6.0 + t27032 / 3.0 + t27037 / 3.0 - 6.0 * t27041 + 2.0 / 3.0 * t27045 - t27049 / 2.0 - t27051 / 9.0 + t27055 - t23616 / 12.0 - t23629 / 3.0 - 3.0 * t27060 - t23650 / 18.0;
    (t27053, t27055, t27059, t27060, t27063)
}
