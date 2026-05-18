//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 921/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk921<F: Float>(t1743: F, t3351: F, t498: F, t511: F, t7231: F, t39199: F, t8571: F, t39183: F, t40705: F, t1981: F, t3142: F, t626: F, t8512: F) -> (F, F, F, F, F) {
    let t45361 = t3351 * t7231 * t511 * t1743 * t498;
    let t45363 = t8571 * t39199;
    let t45365 = t8571 * t39183;
    let t45367 = t8571 * t40705;
    let t45371 = t8512 * t1981 * t3142 * t626;
    (t45361, t45363, t45365, t45367, t45371)
}
