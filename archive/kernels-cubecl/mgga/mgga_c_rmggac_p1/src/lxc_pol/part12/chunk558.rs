//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 558/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk558<F: Float>(t236: F, t7455: F, t1971: F, t7453: F, t209: F, t476: F, t498: F) -> (F, F, F) {
    let t7456 = t236 * t7455;
    let t7457 = t1971 * t7456;
    let t7458 = t7453 * t7457;
    let t7459 = F::cast_from(0.1064114997332445985e-4_f64) * t7458;
    let t7461 = t498 * t476 * t209;
    (t7457, t7459, t7461)
}
