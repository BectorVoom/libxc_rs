//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1086/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1086<F: Float>(t3579: F, t42234: F, t11506: F, t42318: F, t3719: F, t983: F, t11002: F, t3269: F, t3245: F, t3270: F, t39030: F, t1115: F, t3016: F, t10667: F, t11342: F, t42389: F) -> (F, F, F, F, F, F) {
    let t44560 = t3579 * t42234 / 2.0;
    let t44562 = 3.0 / 2.0 * t11506 * t42318;
    let t44563 = t3719 * t983;
    let t44564 = t11002 * t44563;
    let t44566 = 5.0 / 8.0 * t3269 * t44564;
    let t44568 = t3270 * t39030 * t3245;
    let t44570 = t3269 * t44568 / 2.0;
    let t44572 = t3270 * t1115 * t3016;
    let t44574 = 3.0 / 4.0 * t10667 * t44572;
    let t44576 = 3.0 / 4.0 * t42389 * t11342;
    (t44560, t44562, t44566, t44570, t44574, t44576)
}
