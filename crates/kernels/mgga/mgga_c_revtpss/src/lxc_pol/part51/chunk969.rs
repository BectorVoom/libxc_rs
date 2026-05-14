//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 969/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk969<F: Float>(t2322: F, t33591: F, t25082: F, t27153: F, t36970: F, t32322: F, t7937: F, t13648: F, t2014: F, t8595: F, t125483: F, t125486: F, t125488: F, t125491: F, t125495: F, t125497: F, t125499: F, t125500: F, t125502: F, t125505: F, t125507: F, t125510: F, t125512: F, t125514: F, t125515: F) -> (F,) {
    let t125517 = t2322 * t33591;
    let t125521 = 3.0 * t25082 * t36970 * t27153;
    let t125522 = t32322 * t7937;
    let t125525 = t2014 * t8595 * t13648;
    let t125526 = -t125483 + t125486 - t125488 - t125491 + t125495 + 12.0 * t125497 - t125499 - 2.0 * t125500 + 6.0 * t125502 - t125505 - t125507 + t125510 + t125512 - t125514 - 4.0 * t125515 - 4.0 * t125517 - t125521 - 2.0 * t125522 - t125525;
    (t125526,)
}
