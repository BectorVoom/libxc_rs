//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1008/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1008<F: Float>(t26768: F, t525: F, t165: F, t28: F, t3565: F, t5935: F, t1360: F, t3588: F, t1058: F, t1359: F, t1969: F, t379: F, t376: F, t6588: F, t1349: F, t1984: F, t6615: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26769 = t525 * t26768;
    let t26770 = t26769 * t165;
    let t26771 = t28 * t26770;
    let t26777 = t5935 * t3565;
    let t26779 = t1360 * t3588;
    let t26780 = t28 * t26779;
    let t26783 = t1359 * t1058;
    let t26785 = t1969 * t26783 * t379;
    let t26788 = t376 * t6588;
    let t26789 = t1349 * t26788;
    let t26791 = t1984 * t6615;
    (t26769, t26770, t26771, t26777, t26779, t26780, t26783, t26785, t26788, t26789, t26791)
}
