//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1086/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1086<F: Float>(t174: F, t7815: F, t4257: F, t7450: F, t301: F, t4256: F, t8539: F, t2030: F, t372: F, t4262: F, t17752: F, t8923: F) -> (F, F, F, F) {
    let t34903 = t7815 * t174;
    let t34905 = t7450 * t34903 * t4257;
    let t34909 = t7450 * t4256 * t8539 * t301;
    let t34913 = t2030 * t4262 * t8539 * t372;
    let t34916 = t2030 * t17752 * t8923;
    (t34905, t34909, t34913, t34916)
}
