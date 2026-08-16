//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 870/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk870<F: Float>(t12586: F, t3459: F, t1172: F, t4180: F, t1092: F, t3670: F, t1098: F, t1108: F, t3700: F, t1426: F, t175: F, t384: F, t879: F, t922: F) -> (F, F, F, F, F, F) {
    let t12587 = t12586 * t3459;
    let t12589 = t4180 * t1172;
    let t12599 = t3670 * t1092;
    let t12601 = t3670 * t1098;
    let t12603 = t3700 * t1108;
    let t12608 = t384 * t1426 * t175 * t922 * t879;
    (t12587, t12589, t12599, t12601, t12603, t12608)
}
