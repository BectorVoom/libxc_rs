//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1067/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1067<F: Float>(t25901: F, t25904: F, t1955: F, t4066: F, t212: F, t7274: F, t1358: F, t689: F, t2022: F, t785: F, t2439: F, t1032: F, t1419: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25905 = t25904 * t25901;
    let t25909 = t1955 * t4066;
    let t25912 = t212 * t7274;
    let t25913 = t25912 * t1358;
    let t25914 = t689 * t25913;
    let t25916 = t785 * t2022;
    let t25917 = t25916 * t1358;
    let t25919 = 0.65049603595885220126e-3 * t2439 * t25917;
    let t25920 = t1419 * t1032;
    let t25921 = t1955 * t25920;
    (t25905, t25909, t25912, t25913, t25914, t25916, t25917, t25919, t25920, t25921)
}
