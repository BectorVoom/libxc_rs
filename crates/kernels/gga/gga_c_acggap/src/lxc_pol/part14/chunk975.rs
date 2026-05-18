//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 975/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk975<F: Float>(t2268: F, t30456: F, t1562: F, t30948: F, t1444: F, t1992: F, t30154: F, t7586: F, t1350: F, t30147: F, t5129: F, t7647: F) -> (F, F, F, F, F) {
    let t34510 = t30456 * t2268;
    let t34512 = t30948 * t1562;
    let t34513 = F::new(0.16006300097412701803e-1) * t34512;
    let t34516 = t30154 * t7586 * t1992 * t1444;
    let t34526 = t30147 * t7586 * t1992 * t1350;
    let t34534 = t7647 * t5129;
    (t34510, t34513, t34516, t34526, t34534)
}
