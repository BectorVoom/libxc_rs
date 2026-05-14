//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1165/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1165<F: Float>(t47672: F, t9590: F, t2014: F, t2034: F, t13625: F, t25082: F, t32113: F, t26088: F, t531: F, t7238: F, t10194: F, t10259: F, t10260: F, t10415: F, t118: F, t1310: F, t1453: F, t2007: F, t2322: F, t25078: F, t25169: F, t25835: F, t508: F, t651: F, t671: F, t6985: F, t92719: F, t92724: F, t92727: F, t92731: F, t92733: F, t92736: F, t92737: F, t94224: F, t94336: F, t94341: F, t94348: F) -> (F,) {
    let t94349 = t47672 * t9590;
    let t94352 = 6.0 * t2014 * t2034 * t94349;
    let t94355 = 18.0 * t25082 * t32113 * t13625;
    let t94358 = t531 * t26088;
    let t94361 = 9.0 * t2014 * t94358 * t7238;
    let t94365 = -t92719 * t508 - 6.0 * t2322 * t25078 - t92724 - t92727 - 2.0 * t6985 * t10260 - t92731 - t92733 - t92736 - 6.0 * t92737 * t671 - 6.0 * t10194 * t2007 - t118 * (t94224 + t94336) - t94341 - 2.0 * t651 * t2007 * t10259 + t94348 - t94352 - t94355 + 3.0 * t25835 * t1453 + t94361 - 3.0 * t25169 * t1310 - t10415 * t2007;
    (t94365,)
}
