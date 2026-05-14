//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1061/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1061<F: Float>(t1115: F, t39197: F, t42878: F, t39190: F, t42882: F, t11002: F, t3229: F, t3269: F, t11336: F, t3232: F, t3270: F, t3579: F, t41190: F, t3262: F, t3465: F, t43950: F) -> (F, F, F, F, F, F) {
    let t44061 = 15.0 / 4.0 * t39197 * t1115 * t42878;
    let t44064 = 135.0 / 32.0 * t39190 * t1115 * t42882;
    let t44066 = t11002 * t1115 * t3229;
    let t44068 = 5.0 / 16.0 * t3269 * t44066;
    let t44070 = t3270 * t11336 * t3232;
    let t44072 = t3269 * t44070 / 4.0;
    let t44074 = 5.0 / 8.0 * t3579 * t41190;
    let t44077 = 3.0 / 4.0 * t3262 * t3465 * t43950;
    (t44061, t44064, t44068, t44072, t44074, t44077)
}
