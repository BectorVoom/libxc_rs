//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1059/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1059<F: Float>(t11555: F, t12098: F, t3275: F, t11486: F, t3262: F, t11506: F, t41337: F, t3579: F, t41816: F, t12811: F, t1561: F, t3277: F, t11550: F, t12056: F, t11189: F, t43979: F) -> (F, F, F, F, F, F, F) {
    let t44032 = 5.0 / 8.0 * t3275 * t12098 * t11555;
    let t44035 = 15.0 / 8.0 * t3262 * t12098 * t11486;
    let t44037 = 3.0 / 2.0 * t11506 * t41337;
    let t44039 = 5.0 / 8.0 * t3579 * t41816;
    let t44040 = t1561 * t12811;
    let t44043 = 5.0 / 16.0 * t3275 * t44040 * t3277;
    let t44046 = 3.0 / 2.0 * t3262 * t12056 * t11550;
    let t44049 = 45.0 / 32.0 * t3275 * t11189 * t43979;
    (t44032, t44035, t44037, t44039, t44043, t44046, t44049)
}
