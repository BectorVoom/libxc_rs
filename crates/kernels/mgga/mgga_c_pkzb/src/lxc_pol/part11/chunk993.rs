//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 993/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk993<F: Float>(t154: F, t18060: F, t276: F, t655: F, t486: F, t779: F, t148: F, t179: F, t299: F, t5722: F, t768: F, t46: F, t5953: F, t5719: F, t5932: F, t2003: F, t67: F) -> (F, F, F, F, F, F, F, F) {
    let t18063 = t276 * t154 * t18060 * t655;
    let t18086 = t486 * t779;
    let t18107 = t148 * t779;
    let t18110 = t299 * t179 * t18107 * t655;
    let t18152 = t768 * t5722;
    let t18153 = t18152 * t46;
    let t18154 = t5953 * t18153;
    let t18160 = t5719 * t18153;
    let t18163 = t5932 * t18153;
    let t18182 = t67 * t2003;
    (t18063, t18086, t18107, t18110, t18154, t18160, t18163, t18182)
}
