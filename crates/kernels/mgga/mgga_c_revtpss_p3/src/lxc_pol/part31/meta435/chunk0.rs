//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1555/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1555<F: Float>(t19680: F, t4806: F, t1042: F, t5819: F, t999: F, t1032: F, t6235: F, t1040: F, t5825: F, t4872: F, t1651: F, t905: F) -> (F, F, F, F, F, F, F) {
    let t19687 = t4806 * t19680;
    let t19688 = t1042 * t19687;
    let t19691 = t5819 * t999;
    let t19692 = t4806 * t19691;
    let t19693 = t1042 * t19692;
    let t19696 = t6235 * t1032;
    let t19697 = t19696 * t1040;
    let t19700 = t5825 * t999;
    let t19701 = t4872 * t19700;
    let t19702 = t1042 * t19701;
    let t19705 = t1651 * t905;
    (t19688, t19691, t19693, t19696, t19697, t19702, t19705)
}
