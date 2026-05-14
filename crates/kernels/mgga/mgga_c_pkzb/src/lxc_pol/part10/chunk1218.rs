//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1218/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1218<F: Float>(t19107: F, t22971: F, t19116: F, t3185: F, t6475: F, t8350: F, t3206: F, t8354: F, t8450: F, t8452: F, t926: F, t300: F, t3199: F, t931: F, t2099: F, t8311: F, t918: F) -> (F, F, F, F, F, F, F) {
    let t23075 = t19107 * t22971;
    let t23081 = t19116 * t22971;
    let t23088 = t3185 * t6475 * t8350;
    let t23091 = t3206 * t6475 * t8354;
    let t23122 = t8450 * t926 * t8452;
    let t23130 = t300 * t931 * t3199;
    let t23149 = t918 * t2099 * t8311;
    (t23075, t23081, t23088, t23091, t23122, t23130, t23149)
}
