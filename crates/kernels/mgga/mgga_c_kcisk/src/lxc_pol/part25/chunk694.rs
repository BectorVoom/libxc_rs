//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 694/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk694<F: Float>(t2564: F, t5278: F, t1936: F, t2567: F, t1935: F, t1931: F, t2568: F, t5061: F, t5320: F) -> (F, F, F, F, F) {
    let t7422 = t5278 * t2564;
    let t7424 = t2567 * t1936;
    let t7425 = t1935 * t7424;
    let t7427 = t1931 * t2568;
    let t7429 = t5061 * t5320;
    (t7422, t7424, t7425, t7427, t7429)
}
