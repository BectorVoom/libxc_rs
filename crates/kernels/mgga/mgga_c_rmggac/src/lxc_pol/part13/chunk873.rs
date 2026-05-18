//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 873/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk873<F: Float>(t1986: F, t5160: F, t675: F, t2191: F, t8587: F, t26857: F, t7518: F, t6355: F, t7521: F, t1240: F, t236: F, t3352: F, t551: F, t7230: F) -> (F, F, F, F, F) {
    let t39418 = t675 * t1986 * t5160;
    let t39420 = t2191 * t8587;
    let t39423 = t26857 * t7518;
    let t39425 = t6355 * t7521;
    let t39433 = t7230 * t3352 * t236 * t551 * t1240;
    (t39418, t39420, t39423, t39425, t39433)
}
