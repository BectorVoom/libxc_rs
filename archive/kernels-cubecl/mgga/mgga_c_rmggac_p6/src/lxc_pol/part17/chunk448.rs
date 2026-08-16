//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 448/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk448<F: Float>(t4698: F, t4700: F, t1664: F, t356: F, t108: F, t1539: F) -> (F, F, F, F) {
    let t4997 = F::cast_from(1584.0_f64) * t4698;
    let t4998 = F::cast_from(1872.0_f64) * t4700;
    let t5002 = t1664 * t356;
    let t5011 = t1539 * t108;
    (t4997, t4998, t5002, t5011)
}
