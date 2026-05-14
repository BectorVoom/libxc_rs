//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 738/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk738<F: Float>(t17630: F, t17636: F, t643: F, t12116: F, t17631: F, t15734: F, t15739: F, t15744: F, t15748: F, t15750: F, t15754: F, t15758: F, t15760: F, t15765: F, t15770: F, t15774: F) -> (F, F, F) {
    let t17638 = t17636 * t17630 * t643;
    let t17641 = t12116 * t17631;
    let t17666 = 0.3209574074074074074e-1 * t15734 - 0.1604787037037037037e0 * t15739 + 0.57772333333333333332e0 * t15744 - 0.38514888888888888888e0 * t15748 - 0.9628722222222222222e-1 * t15750 - 0.86658499999999999998e0 * t15754 + 0.11554466666666666666e1 * t15758 + 0.4814361111111111111e-1 * t15760 - 0.9628722222222222222e-1 * t15765 + 0.28886166666666666666e0 * t15770 - 0.14443083333333333333e0 * t15774;
    (t17638, t17641, t17666)
}
