//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 525/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk525<F: Float>(t3330: F, t3331: F, t1171: F, t1175: F, t1170: F, t1169: F, t284: F) -> (F, F, F, F) {
    let t3333 = F::new(2.0) * t3330 * t3331;
    let t3334 = t1175 * t1171;
    let t3335 = t1170 * t3334;
    let t3337 = t1169 * t284;
    (t3333, t3334, t3335, t3337)
}
