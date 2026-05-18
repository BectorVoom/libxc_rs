//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 765/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk765<F: Float>(t1083: F, t398: F, t5814: F, t1524: F, t506: F, t1713: F, t322: F, t1426: F, t175: F, t384: F, t1841: F, t935: F) -> (F, F, F, F, F, F) {
    let t5816 = t398 * t1083 * t5814;
    let t5819 = t506 * t1524;
    let t5821 = t398 * t1083 * t5819;
    let t5824 = t1713 * t322;
    let t5826 = t1426 * t175 * t5824;
    let t5827 = t384 * t5826;
    let t5829 = t935 * t1841;
    (t5816, t5819, t5821, t5826, t5827, t5829)
}
