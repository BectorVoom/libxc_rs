//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1113/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1113<F: Float>(t27048: F, t27101: F, t27176: F, t305: F, t321: F, t326: F, t352: F, t36013: F, t37584: F, t41463: F, t42637: F, t43065: F, t43658: F, t43692: F, t44183: F, t44194: F, t5148: F, t5266: F, t793: F, t794: F, t839: F, t876: F, t9523: F, t9551: F) -> F {
    let t44230 = -F::new(0.5987120850931904282e-1) * t41463 - F::new(0.23948483403727617128e0) * t5148 * t44183 * t321 + F::new(0.71845450211182851384e0) * t27048 * t42637 + F::new(0.23948483403727617128e0) * t36013 + t37584 + F::new(0.59871208509319042821e-1) * t305 * t43658 - F::new(0.59871208509319042821e-1) * t326 * t43065 - F::new(0.23948483403727617128e0) * t27101 * t9551 * t794 - F::new(0.47896966807455234256e0) * t27176 * t9551 * t839 + F::new(0.23948483403727617128e0) * t5266 * t44194 * t352 + F::new(0.11974241701863808564e0) * t793 * t43692 + F::new(0.35922725105591425692e0) * t27048 * t9523 * t876;
    t44230
}
