//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1095/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1095<F: Float>(t19508: F, t19622: F, t4867: F, t144: F, t5698: F, t203: F, t9078: F, t19507: F, t4017: F, t681: F, t1266: F, t186: F) -> (F, F, F, F, F) {
    let t19624 = t19508 * t19622 * t4867;
    let t19636 = t144 * t5698;
    let t19639 = t19636 * t203 * t19622 * t9078;
    let t19644 = t19507 * t681 * t19622 * t4017;
    let t19652 = t1266 * t186;
    (t19624, t19636, t19639, t19644, t19652)
}
