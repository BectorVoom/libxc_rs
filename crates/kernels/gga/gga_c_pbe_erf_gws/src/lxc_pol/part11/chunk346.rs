//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 346/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk346<F: Float>(t1339: F, t169: F, t242: F, t700: F, t766: F, t145: F, t34: F, t532: F, t1336: F, t6: F, t837: F) -> (F, F, F, F, F, F, F) {
    let t1342 = 0.14149184788746388121e0 * t169 * t1339 * t242;
    let t1349 = 0.1061188859155979109e0 * t169 * t766 * t700;
    let t1350 = 2.0 * t145;
    let t1351 = t34 * t532;
    let t1352 = 8.0 * t1351;
    let t1353 = 6.0 * t1336;
    let t1365 = t6 * t837;
    (t1342, t1349, t1350, t1351, t1352, t1353, t1365)
}
