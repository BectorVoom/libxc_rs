//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 814/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk814<F: Float>(t11235: F, t4018: F, t11234: F, t619: F, t640: F, t2941: F, t128: F, t200: F, t1954: F, t2922: F, t2903: F, t3635: F, t1459: F, t8286: F, t475: F, t4855: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11236 = t11235 * t4018;
    let t11237 = t11234 * t11236;
    let t11239 = t640 * t619;
    let t11240 = t2941 * t11239;
    let t11242 = t128 * t200;
    let t11243 = t11242 * t1954;
    let t11244 = t2922 * t11243;
    let t11246 = t2903 * t3635;
    let t11248 = t8286 * t1459;
    let t11249 = t475 * t4855;
    (t11236, t11237, t11239, t11240, t11242, t11243, t11244, t11246, t11248, t11249)
}
