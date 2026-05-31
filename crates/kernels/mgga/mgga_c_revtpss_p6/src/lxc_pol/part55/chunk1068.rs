//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1068/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1068<F: Float>(t32698: F, t32732: F, t532: F, t1450: F, t2014: F, t1353: F, t2033: F, t26405: F, t25082: F, t2042: F, t7547: F, t2113: F, t7331: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32733 = t32698 + t32732;
    let t32734 = t532 * t32733;
    let t32735 = t32734 * t1450;
    let t32736 = t2014 * t32735;
    let t32737 = t2033 * t1353;
    let t32738 = t26405 * t32737;
    let t32740 = F::cast_from(3.0_f64) * t25082 * t32738;
    let t32760 = F::cast_from(3.0_f64) * t7547 * t2042;
    let t32762 = F::cast_from(6.0_f64) * t2113 * t7331;
    (t32733, t32734, t32735, t32736, t32737, t32738, t32740, t32760, t32762)
}
