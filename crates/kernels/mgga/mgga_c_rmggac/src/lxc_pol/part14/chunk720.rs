//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 720/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk720<F: Float>(t1255: F, t1986: F, t1034: F, t132: F, t7933: F, t7934: F, t303: F, t388: F, t357: F, t7334: F, t7932: F, t7936: F, t2190: F, t678: F, t7920: F, t2160: F, t49: F) -> (F, F, F, F, F, F, F, F) {
    let t36895 = t1986 * t1255;
    let t36902 = t7933 * t7934 * t1034 * t132;
    let t36906 = t7933 * t7934 * t388 * t303;
    let t36910 = t7933 * t7934 * t388 * t357;
    let t36912 = t7334 * t7932;
    let t36913 = t36912 * t7936;
    let t36916 = t2190 * t7920 * t678;
    let t36920 = t2160 * t49;
    (t36895, t36902, t36906, t36910, t36912, t36913, t36916, t36920)
}
