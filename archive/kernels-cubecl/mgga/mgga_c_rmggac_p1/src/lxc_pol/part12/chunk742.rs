//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 742/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk742<F: Float>(t7754: F, t930: F, t2010: F, t7756: F, t118: F, t2001: F, t353: F, t498: F, t1212: F, t128: F, t1986: F, t209: F) -> (F, F, F) {
    let t35000 = t7754 * t930;
    let t35002 = t2010 * t35000 * t7756;
    let t35018 = t2001 * t118 * t353 * t498;
    let t35024 = t1986 * t118 * t128 * t1212 * t209;
    (t35002, t35018, t35024)
}
