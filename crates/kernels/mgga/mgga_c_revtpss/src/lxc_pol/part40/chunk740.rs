//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 740/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk740<F: Float>(t1312: F, t2320: F, t2322: F, t2327: F, t2371: F, t670: F, t93: F, t1330: F, t72: F, t757: F, t530: F, t566: F, t1353: F) -> (F, F, F, F, F, F) {
    let t3821 = 2.0 * t1312 * t2371 + 4.0 * t2322 * t670 + 2.0 * t2327 * t93 + t2320;
    let t3825 = t1330 * t72;
    let t3826 = t3825 * t757;
    let t3827 = 0.36622894612013090108e-3 * t3826;
    let t3828 = t530 * t566;
    let t3829 = t1353 * t1353;
    (t3821, t3825, t3826, t3827, t3828, t3829)
}
