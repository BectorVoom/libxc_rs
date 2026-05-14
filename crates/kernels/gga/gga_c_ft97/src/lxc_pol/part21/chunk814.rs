//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 814/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk814<F: Float>(t1564: F, t3052: F, t5502: F, t15593: F, t2: F, t4: F, t26: F) -> (F, F, F) {
    let t25579 = t1564 * t5502 * t3052;
    let t25582 = t15593 * t2;
    let t25583 = t25582 * t4;
    let t25584 = t25583 * t26;
    (t25579, t25583, t25584)
}
