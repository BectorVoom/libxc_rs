//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 598/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk598<F: Float>(t609: F, t2104: F, t4455: F, t1610: F, t6176: F, t6136: F) -> (F, F, F, F) {
    let t614 = F::cast_from(0.0_f64) < t609;
    let t6177 = t4455 * t2104;
    let t6178 = t6177 * t1610;
    let t6179 = t6176 * t6178;
    let t6183 = piecewise3::<F>(t614, t6136, -t6136);
    (t6177, t6178, t6179, t6183)
}
