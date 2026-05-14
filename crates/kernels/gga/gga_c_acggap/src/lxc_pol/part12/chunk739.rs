//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 739/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk739<F: Float>(t7337: F, t8783: F, t1165: F, t5606: F, t7351: F, t2068: F, t524: F, t944: F, t406: F) -> (F, F, F, F, F) {
    let t8784 = t7337 * t8783;
    let t8787 = t1165 * t7351 * t5606;
    let t8788 = t2068 * t8787;
    let t8790 = t524 * t944;
    let t8791 = t8790 * t406;
    (t8784, t8787, t8788, t8790, t8791)
}
