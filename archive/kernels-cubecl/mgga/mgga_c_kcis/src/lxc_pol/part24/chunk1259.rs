//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1259/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1259<F: Float>(t1014: F, t28919: F, t28973: F, t19727: F, t3200: F, t95926: F, t19711: F, t4554: F, t1087: F, t303: F, t6556: F, t19656: F, t356: F) -> (F, F, F, F, F, F) {
    let t100578 = t1014 * t28919;
    let t100580 = t1014 * t28973;
    let t100583 = t3200 * t95926 * t19727;
    let t100586 = t4554 * t95926 * t19711;
    let t100596 = t303 * t1087 * t6556;
    let t100599 = t303 * t356 * t19656;
    (t100578, t100580, t100583, t100586, t100596, t100599)
}
