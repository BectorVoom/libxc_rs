//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 969/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk969<F: Float>(t1710: F, t935: F, t7290: F, t296: F, t7112: F, t830: F, t21488: F, t314: F, t805: F, t1880: F, t2610: F, t16534: F, t169: F, t7322: F, t747: F, t20157: F, t322: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21783 = t935 * t1710;
    let t21784 = t7290 * t21783;
    let t21794 = t296 * t7112;
    let t21888 = t830 * t935;
    let t22008 = t21488 * t805 * t314;
    let t22044 = t935 * t1880;
    let t22045 = t2610 * t22044;
    let t22090 = t16534 * t169;
    let t22139 = t7322 * t747;
    let t22144 = t805 * t322 * t20157;
    (t21783, t21784, t21794, t21888, t22008, t22044, t22045, t22090, t22139, t22144)
}
