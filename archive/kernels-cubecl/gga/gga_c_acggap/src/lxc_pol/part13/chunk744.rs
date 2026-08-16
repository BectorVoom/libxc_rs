//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 744/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk744<F: Float>(t7815: F, t961: F, t2030: F, t2031: F, t361: F, t1170: F, t7646: F) -> (F, F, F, F, F) {
    let t7816 = t7815 * t961;
    let t7817 = t2030 * t7816;
    let t7819 = t361 * t2031;
    let t7820 = t2030 * t7819;
    let t7822 = t1170 * t7646;
    (t7816, t7817, t7819, t7820, t7822)
}
