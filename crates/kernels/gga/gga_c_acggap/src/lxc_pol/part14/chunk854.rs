//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 854/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk854<F: Float>(t29991: F, t621: F, t2122: F, t394: F, t2130: F, t7923: F, t2149: F, t7922: F, t861: F, t2140: F, t3054: F, t609: F, t865: F) -> (F, F, F, F, F, F) {
    let t29992 = t29991 * t621;
    let t29997 = t394 * t2122;
    let t30005 = t7923 * t2130;
    let t30006 = t30005 * t2149;
    let t30009 = t7922 * t861 * t2130;
    let t30011 = F::cast_from(0.52041769129231196772e1_f64) * t30009 * t2140;
    let t30015 = F::cast_from(0.39512695097613069591e1_f64) * t3054 * t609 * t865;
    (t29992, t29997, t30005, t30006, t30011, t30015)
}
