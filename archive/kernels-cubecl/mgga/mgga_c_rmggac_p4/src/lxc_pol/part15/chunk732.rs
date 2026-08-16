//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 732/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk732<F: Float>(t305: F, t4616: F, t326: F, t128: F, t25525: F, t338: F, t3839: F, t793: F, t874: F, t838: F, t1540: F, t325: F) -> (F, F, F, F, F, F, F) {
    let t27048 = t305 * t4616;
    let t27055 = t326 * t4616;
    let t27091 = t25525 * t128;
    let t27094 = t3839 * t338;
    let t27101 = t793 * t874;
    let t27176 = t838 * t874;
    let t28295 = t1540 * t325;
    (t27048, t27055, t27091, t27094, t27101, t27176, t28295)
}
