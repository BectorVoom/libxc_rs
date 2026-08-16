//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 692/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk692<F: Float>(t1095: F, t1426: F, t7470: F, t598: F, t1024: F, t19: F, t336: F) -> (F, F, F, F) {
    let t7472 = t1426 * t1095 * t7470;
    let t7473 = t598 * t7472;
    let t7475 = t1024 * t19;
    let t7476 = t7475 * t336;
    (t7472, t7473, t7475, t7476)
}
