//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 555/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk555<F: Float>(t1307: F, t5632: F, t1395: F, t1394: F, t1397: F, t5752: F, t1947: F, t3738: F, t1392: F, t4992: F, t86: F) -> (F, F, F, F, F, F, F, F) {
    let t5769 = t5632 * t1307;
    let t5770 = t1395 * t5769;
    let t5771 = t1394 * t5770;
    let t5773 = t5752 * t1397;
    let t5774 = t1394 * t5773;
    let t5776 = t3738 * t1947;
    let t5777 = t1394 * t5776;
    let t5780 = t86 * t4992 * t1392;
    (t5769, t5770, t5771, t5773, t5774, t5776, t5777, t5780)
}
