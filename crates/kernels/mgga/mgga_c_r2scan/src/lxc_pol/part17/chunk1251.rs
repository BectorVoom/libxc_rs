//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1251/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1251<F: Float>(t12811: F, t498: F, t3275: F, t3352: F, t3579: F, t42234: F, t11506: F, t42318: F, t3719: F, t983: F, t11002: F, t3269: F) -> (F, F, F, F, F) {
    let t44555 = t498 * t12811;
    let t44558 = t3275 * t44555 * t3352 / F::new(4.0);
    let t44560 = t3579 * t42234 / F::new(2.0);
    let t44562 = F::new(3.0) / F::new(2.0) * t11506 * t42318;
    let t44563 = t3719 * t983;
    let t44564 = t11002 * t44563;
    let t44566 = F::new(5.0) / F::new(8.0) * t3269 * t44564;
    (t44555, t44558, t44560, t44562, t44566)
}
