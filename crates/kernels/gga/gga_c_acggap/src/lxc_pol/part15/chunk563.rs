//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 563/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk563<F: Float>(t1165: F, t4267: F, t5606: F, t4643: F, t1181: F, t406: F, t506: F, t157: F) -> (F, F, F) {
    let t5608 = t1165 * t4267 * t5606;
    let t5611 = t4643 * t5606;
    let t5612 = t1181 * t5611;
    let t5615 = t506 * t406;
    let t5616 = t5615 * t157;
    (t5608, t5612, t5616)
}
