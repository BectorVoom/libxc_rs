//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1343/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1343<F: Float>(t117845: F, t117889: F, t118039: F, t118083: F, t2204: F, t5808: F, t1921: F, t8330: F, t1913: F, t8349: F, t31512: F, t571: F, t117168: F, t117170: F, t1464: F, t18178: F, t18217: F, t2205: F, t2212: F, t3: F, t31205: F, t31464: F, t4168: F, t575: F, t8331: F, t8417: F) -> (F, F) {
    let t118085 = t117845 + t117889 + t118039 + t118083;
    let t118089 = 2.0 * t2204 * t5808;
    let t118091 = 2.0 * t8330 * t1921;
    let t118094 = 2.0 * t1913 * t8349;
    let t118099 = 2.0 * t571 * t31512;
    let t118100 = t118085 * t3 * t575 + 2.0 * t1464 * t31464 + t18178 * t2212 + t18217 * t2205 + t1921 * t31205 + t4168 * t8417 + 2.0 * t5808 * t8331 + t117168 + t117170 + t118089 + t118091 + t118094 + t118099;
    (t118085, t118100)
}
