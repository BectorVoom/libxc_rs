//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 756/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk756<F: Float>(t13208: F, t16671: F, t13212: F, t16675: F, t1882: F, t4833: F, t4747: F, t4743: F, t4735: F, t4739: F, t376: F, t4792: F, t89: F, t12889: F, t12890: F, t16668: F, t16673: F, t16677: F, t16679: F, t16684: F, t16689: F, t16692: F, t16696: F, t16699: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17426 = t13208 * t16671;
    let t17429 = t13212 * t16675;
    let t17432 = t1882 * t4833;
    let t17434 = t1882 * t4747;
    let t17436 = t1882 * t4743;
    let t17438 = t1882 * t4735;
    let t17440 = t1882 * t4739;
    let t17443 = t89 * t376 * t4792;
    let t17454 = -4.0 / 9.0 * t16668 - 4.0 / 9.0 * t16673 + 4.0 / 27.0 * t16677 - 2.0 / 27.0 * t16679 + t16684 / 9.0 - 2.0 / 9.0 * t16689 + 8.0 / 9.0 * t16692 + t16696 / 9.0 + 2.0 / 9.0 * t16699 - t12889 - t12890;
    (t17426, t17429, t17432, t17434, t17436, t17438, t17440, t17443, t17454)
}
