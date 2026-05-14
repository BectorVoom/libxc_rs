//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1050/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1050<F: Float>(t25672: F, t3151: F, t3304: F, t3318: F, t7168: F, t1035: F, t7135: F, t1043: F, t1089: F, t3133: F, t1976: F, t3046: F, t994: F, t11199: F, t1981: F, t7143: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25674 = t25672 * t3151 * t3304;
    let t25678 = t7168 * t3151 * t3318;
    let t25681 = t1035 * t7135;
    let t25683 = t25681 * t1043 * t1089;
    let t25687 = t7168 * t3133 * t1089;
    let t25692 = t3046 * t1976;
    let t25695 = t994 * t7135;
    let t25698 = t1981 * t11199;
    let t25699 = t25698 * t7143;
    (t25674, t25678, t25681, t25683, t25687, t25692, t25695, t25698, t25699)
}
