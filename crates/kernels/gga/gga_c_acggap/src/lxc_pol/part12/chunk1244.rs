//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1244/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1244<F: Float>(t33551: F, t7963: F, t8306: F, t33787: F, t2131: F, t2147: F, t2394: F, t847: F, t33574: F, t8085: F, t7987: F, t9159: F) -> (F, F, F, F, F) {
    let t38441 = F::new(0.17347256376410398924e1) * t7963 * t8306 * t33551;
    let t38443 = t7963 * t8306 * t33787;
    let t38453 = t2131 * t2147 * t2394 * t847;
    let t38455 = t33574 * t8085;
    let t38458 = F::new(0.34694512752820797848e1) * t7987 * t9159;
    (t38441, t38443, t38453, t38455, t38458)
}
