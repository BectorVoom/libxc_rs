//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 838/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk838<F: Float>(t114: F, t27879: F, t27907: F, t27984: F, t28017: F, t532: F, t1450: F, t2014: F, t1513: F, t25823: F, t665: F, t25826: F, t4287: F, t6998: F, t25822: F, t25824: F) -> (F, F, F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t28019 = t27879 + t27907 + t27984 + t28017;
    let t28020 = t532 * t28019;
    let t28021 = t28020 * t1450;
    let t28022 = t2014 * t28021;
    let t28034 = t25823 * t1513;
    let t28036 = t1513 * t665;
    let t28037 = t25826 * t28036;
    let t28039 = t6998 * t4287;
    let t28042 = piecewise3(t115, 0.0, t25822 + t25824 / 3.0 + t28034 / 3.0 + t28037 / 4.0 - t28039 / 8.0);
    (t28019, t28021, t28022, t28034, t28037, t28039, t28042)
}
