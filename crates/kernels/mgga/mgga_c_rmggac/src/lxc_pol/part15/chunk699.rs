//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 699/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk699<F: Float>(t35924: F, t854: F, t305: F, t3899: F, t36172: F, t655: F, t2067: F, t25525: F, t2078: F, t3839: F, t7834: F, t838: F, t25809: F, t664: F, t35583: F, t793: F) -> (F, F, F, F, F, F, F, F) {
    let t36190 = t854 * t35924;
    let t36200 = t305 * t3899;
    let t36201 = 0.22765842247987981715e0 * t36200;
    let t36204 = t655 * t36172;
    let t36205 = 0.51855529564861513904e-1 * t36204;
    let t36250 = t25525 * t2067;
    let t36254 = t3839 * t2078;
    let t36274 = t838 * t7834;
    let t36280 = t25809 * t664;
    let t36284 = t793 * t35583;
    (t36190, t36201, t36205, t36250, t36254, t36274, t36280, t36284)
}
