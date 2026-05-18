//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 502/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk502<F: Float>(t555: F, t785: F, t1358: F, t2439: F, t1419: F, t212: F, t689: F, t1357: F, t1445: F, t2453: F, t556: F, t136: F, t561: F) -> (F, F, F, F, F) {
    let t3895 = t785 * t555;
    let t3896 = t3895 * t1358;
    let t3898 = F::new(0.65049603595885220126e-3) * t2439 * t3896;
    let t3899 = t212 * t1419;
    let t3900 = t3899 * t1358;
    let t3901 = t689 * t3900;
    let t3903 = t1357 * t1445;
    let t3904 = t689 * t3903;
    let t3906 = t2453 * t556;
    let t3907 = t561 * t136;
    (t3898, t3901, t3904, t3906, t3907)
}
