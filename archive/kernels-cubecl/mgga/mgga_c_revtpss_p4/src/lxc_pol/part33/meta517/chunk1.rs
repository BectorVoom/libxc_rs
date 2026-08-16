//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1855/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1855<F: Float>(t196: F, t197: F, t5528: F, t2035: F, t7313: F, t7898: F, t1032: F, t1892: F, t1955: F) -> (F, F, F, F, F) {
    let t27833 = t5528 * t196 * t197;
    let t27834 = t27833 * t2035;
    let t27835 = t7898 * t7313;
    let t27836 = t1892 * t1032;
    let t27837 = t1955 * t27836;
    (t27833, t27834, t27835, t27836, t27837)
}
