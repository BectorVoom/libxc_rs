//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 825/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk825<F: Float>(t1614: F, t3351: F, t511: F, t618: F, t7231: F, t10095: F, t16043: F, t1528: F, t515: F, t570: F, t1652: F, t34828: F, t9864: F, t6477: F, t34884: F, t9845: F) -> (F, F, F, F, F, F, F) {
    let t45451 = t3351 * t7231 * t511 * t618 * t1614;
    let t45453 = t16043 * t10095;
    let t45458 = t3351 * t7231 * t515 * t1528 * t570;
    let t45463 = t3351 * t7231 * t515 * t618 * t1652;
    let t45466 = t34828 * t9864;
    let t45468 = t6477 * t511;
    let t45469 = t45468 * t9864;
    let t45473 = t34884 * t9845;
    (t45451, t45453, t45458, t45463, t45466, t45469, t45473)
}
