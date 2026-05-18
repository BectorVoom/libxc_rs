//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1116/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1116<F: Float>(t7346: F, t7347: F, t8480: F, t31350: F, t4971: F, t7447: F, t8823: F, t7440: F, t8826: F, t1488: F, t2030: F, t2031: F) -> (F, F, F, F, F) {
    let t35844 = t7346 * t8480 * t7347;
    let t35846 = t31350 * t4971;
    let t35848 = t7447 * t8823;
    let t35850 = t7440 * t8826;
    let t35853 = t2030 * t1488 * t2031;
    (t35844, t35846, t35848, t35850, t35853)
}
