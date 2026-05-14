//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1152/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1152<F: Float>(t25586: F, t342: F, t11627: F, t1976: F, t994: F, t11223: F, t27639: F, t995: F, t3151: F, t3153: F, t19482: F, t988: F, t25610: F, t3043: F, t25604: F, t7156: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93867 = t342 * t25586;
    let t93870 = t11627 * t1976;
    let t93881 = t994 * t25586;
    let t93884 = t11223 * t1976;
    let t93890 = t995 * t27639;
    let t93892 = t1976 * t3151 * t3153;
    let t93893 = t19482 * t988;
    let t93897 = t25610 * t27639;
    let t93901 = t3043 * t1976;
    let t93904 = t7156 * t25604;
    (t93867, t93870, t93881, t93884, t93890, t93892, t93893, t93897, t93901, t93904)
}
