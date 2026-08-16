//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 926/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk926<F: Float>(t8507: F, t999: F, t31892: F, t1071: F, t8513: F, t8521: F, t1032: F, t994: F, t8501: F) -> (F, F, F, F, F) {
    let t31904 = t8507 * t999;
    let t31905 = t31892 * t31904;
    let t31908 = t8513 * t1071;
    let t31909 = t31908 * t8521;
    let t31912 = t994 * t1032;
    let t31913 = t31912 * t8501;
    (t31905, t31908, t31909, t31912, t31913)
}
