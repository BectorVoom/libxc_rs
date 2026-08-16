//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1066/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1066<F: Float>(t12621: F, t1280: F, t3634: F, t828: F, t3630: F, t3625: F, t3372: F, t5405: F, t3626: F, t3368: F, t3624: F, t3746: F) -> (F, F, F, F, F) {
    let t12769 = t1280 * t12621;
    let t12772 = t828 * t3634;
    let t12773 = t12772 * t3630;
    let t12774 = t3625 * t12773;
    let t12776 = t3372 * t5405;
    let t12777 = t3626 * t12776;
    let t12780 = t3368 * t5405;
    let t12781 = t3626 * t12780;
    let t12784 = t3746 * t3624;
    (t12769, t12774, t12777, t12781, t12784)
}
