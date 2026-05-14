//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 955/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk955<F: Float>(t34381: F, t34422: F, t34438: F, t34466: F, t3: F, t2042: F, t8245: F, t2170: F, t7950: F, t7953: F, t1918: F, t33996: F, t33998: F, t34000: F, t34003: F, t34006: F, t34009: F, t34011: F, t34014: F, t573: F, t8616: F, t8771: F) -> (F, F, F, F) {
    let t34468 = t34381 + t34422 + t34438 + t34466;
    let t34469 = t3 * t34468;
    let t34477 = param_d * t34468;
    let t34481 = t8245 * t2042;
    let t34483 = t2170 * t7950;
    let t34485 = t2170 * t7953;
    let t34490 = 3.0 * t1918 * t8771 + t34477 * t573 + 3.0 * t33996 + 6.0 * t33998 + 3.0 * t34000 + t34003 + t34006 + t34009 + t34011 + t34014 + 3.0 * t34481 + 6.0 * t34483 + 3.0 * t34485 + t8616;
    (t34468, t34469, t34477, t34490)
}
