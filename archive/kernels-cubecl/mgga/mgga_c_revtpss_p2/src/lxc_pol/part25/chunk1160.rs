//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1160/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1160<F: Float>(t1043: F, t1089: F, t25681: F, t3133: F, t7168: F, t1976: F, t3046: F, t7135: F, t994: F, t11199: F, t1981: F, t7143: F) -> (F, F, F, F, F, F) {
    let t25683 = t25681 * t1043 * t1089;
    let t25687 = t7168 * t3133 * t1089;
    let t25692 = t3046 * t1976;
    let t25695 = t994 * t7135;
    let t25698 = t1981 * t11199;
    let t25699 = t25698 * t7143;
    (t25683, t25687, t25692, t25695, t25698, t25699)
}
