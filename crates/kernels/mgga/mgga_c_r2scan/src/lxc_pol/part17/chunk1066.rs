//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1066/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1066<F: Float>(t3262: F, t3574: F, t41202: F, t12045: F, t40282: F, t12060: F, t40713: F, t3275: F, t3465: F, t42940: F, t39030: F, t40630: F, t43771: F, t3472: F, t43802: F, t12056: F, t3579: F, t495: F, t797: F) -> (F, F, F, F, F, F, F) {
    let t44147 = 3.0 / 2.0 * t3262 * t41202 * t3574;
    let t44150 = 3.0 / 2.0 * t40282 * t12045;
    let t44152 = 5.0 / 8.0 * t40713 * t12060;
    let t44155 = t3275 * t3465 * t42940 / 2.0;
    let t44158 = 3.0 * t40630 * t39030 * t43771;
    let t44161 = 15.0 / 8.0 * t3262 * t3472 * t43802;
    let t44165 = t3579 * t495 * t12056 * t797 / 2.0;
    (t44147, t44150, t44152, t44155, t44158, t44161, t44165)
}
