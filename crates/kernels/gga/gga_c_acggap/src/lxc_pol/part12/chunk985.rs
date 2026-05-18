//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 985/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk985<F: Float>(t119: F, t8301: F, t32199: F, t7963: F, t8306: F, t16020: F, t7942: F, t2217: F, t394: F) -> (F, F, F, F) {
    let t33163 = t119 * t8301;
    let t33170 = t7963 * t8306 * t32199;
    let t33173 = t7942 * t8306 * t16020;
    let t33175 = t394 * t2217;
    (t33163, t33170, t33173, t33175)
}
