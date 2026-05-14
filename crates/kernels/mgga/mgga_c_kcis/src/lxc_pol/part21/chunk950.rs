//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 950/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk950<F: Float>(t2626: F, t7609: F, t113: F, t2585: F, t2588: F, t7617: F, t740: F, t805: F, t2491: F, t2593: F, t774: F, t808: F, t2526: F, t153: F, t160: F, t2150: F, t2605: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26520 = t7609 * t2626;
    let t26521 = t2585 * t113;
    let t26523 = t2588 * t7617;
    let t26525 = t805 * t740;
    let t26527 = t113 * t2491;
    let t26528 = t2593 * t26527;
    let t26530 = t740 * t774;
    let t26531 = t808 * t26530;
    let t26533 = t113 * t2526;
    let t26534 = t808 * t26533;
    let t26536 = t153 * t160;
    let t26538 = t2605 * t2150;
    (t26520, t26521, t26523, t26525, t26527, t26528, t26530, t26531, t26533, t26534, t26536, t26538)
}
