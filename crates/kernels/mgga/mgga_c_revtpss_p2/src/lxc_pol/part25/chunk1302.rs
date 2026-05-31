//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1302/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1302<F: Float>(t530: F, t7311: F, t2014: F, t25865: F, t47672: F, t9590: F, t2034: F, t13625: F, t25082: F, t32113: F, t26088: F, t531: F) -> (F, F, F, F) {
    let t94345 = t530 * t7311;
    let t94348 = F::cast_from(18.0_f64) * t2014 * t94345 * t25865;
    let t94349 = t47672 * t9590;
    let t94352 = F::cast_from(6.0_f64) * t2014 * t2034 * t94349;
    let t94355 = F::cast_from(18.0_f64) * t25082 * t32113 * t13625;
    let t94358 = t531 * t26088;
    (t94348, t94352, t94355, t94358)
}
