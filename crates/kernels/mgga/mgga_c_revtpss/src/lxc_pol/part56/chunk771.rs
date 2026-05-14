//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 771/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk771<F: Float>(t28042: F, t508: F, t651: F, t1843: F, t7002: F, t2322: F, t7742: F, t4254: F, t1310: F, t7741: F, t22496: F, t8717: F, t25082: F, t4237: F, t76: F, t13269: F, t38: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28043 = t508 * t28042;
    let t28045 = 2.0 * t651 * t28043;
    let t28056 = t1843 * t7002;
    let t28058 = 2.0 * t651 * t28056;
    let t28060 = 2.0 * t2322 * t7742;
    let t28062 = 2.0 * t4254 * t7742;
    let t28063 = t1310 * t7741;
    let t28065 = 2.0 * t651 * t28063;
    let t28067 = t8717 * t22496;
    let t28069 = 3.0 * t25082 * t28067;
    let t28089 = t76 * t4237;
    let t28093 = t13269 * t38;
    (t28043, t28045, t28056, t28058, t28060, t28062, t28063, t28065, t28067, t28069, t28089, t28093)
}
