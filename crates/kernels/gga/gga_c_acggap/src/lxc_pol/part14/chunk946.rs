//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 946/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk946<F: Float>(t7884: F, t8396: F, t7887: F, t7990: F, t9076: F, t1620: F, t7973: F, t2331: F, t323: F, t851: F, t2137: F, t32123: F) -> (F, F, F, F, F) {
    let t33682 = t7884 * t8396;
    let t33683 = t33682 * t7887;
    let t33686 = F::new(0.34694512752820797848e1) * t7990 * t9076;
    let t33691 = F::new(0.26341796731742046394e1) * t7973 * t1620;
    let t33695 = t851 * t2331 * t323;
    let t33698 = t2137 * t32123;
    (t33683, t33686, t33691, t33695, t33698)
}
