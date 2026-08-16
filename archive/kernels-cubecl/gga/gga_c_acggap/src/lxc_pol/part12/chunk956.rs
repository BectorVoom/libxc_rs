//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 956/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk956<F: Float>(t151: F, t2116: F, t3668: F, t409: F, t1103: F, t7746: F, t7637: F, t7709: F, t2113: F, t7610: F, t2082: F, t30567: F) -> (F, F, F, F, F, F) {
    let t31643 = t151 * t2116 * t3668;
    let t31644 = t31643 * t409;
    let t31646 = t7746 * t1103;
    let t31658 = t7637 * t7709;
    let t31660 = t7610 * t2113;
    let t31662 = t30567 * t2082;
    (t31643, t31644, t31646, t31658, t31660, t31662)
}
