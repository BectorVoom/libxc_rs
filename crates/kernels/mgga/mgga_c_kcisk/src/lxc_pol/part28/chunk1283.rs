//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1283/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1283<F: Float>(t111036: F, t3373: F, t9368: F, t32637: F, t32643: F, t111048: F, t32661: F, t32672: F, t111077: F, t12769: F, t140: F, t178: F, t981: F, t12697: F, t12699: F, t31976: F, t937: F) -> (F, F, F, F, F, F, F, F) {
    let t111097 = t3373 * t111036 * t9368;
    let t111099 = t32637 * t32643;
    let t111101 = t32661 * t111048;
    let t111103 = t32672 * t32643;
    let t111105 = t111077 * t9368;
    let t111109 = t140 * t178 * t981 * t12769;
    let t111113 = t140 * t178 * t12697 * t12699;
    let t111116 = t140 * t937 * t31976;
    (t111097, t111099, t111101, t111103, t111105, t111109, t111113, t111116)
}
