//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 981/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk981<F: Float>(t11872: F, t3408: F, t1936: F, t7073: F, t1453: F, t291: F, t7949: F, t959: F, t2767: F, t3717: F, t7294: F, t11365: F, t2660: F, t7880: F) -> (F, F, F, F, F, F) {
    let t11873 = t11872 * t3408;
    let t11875 = t7073 * t1936;
    let t11876 = t1453 * t291;
    let t11878 = t11876 * t959 * t7949;
    let t11879 = t11875 * t11878;
    let t11882 = t7294 * t3717 * t2767;
    let t11885 = t2660 * t11365 * t7880;
    (t11873, t11875, t11878, t11879, t11882, t11885)
}
