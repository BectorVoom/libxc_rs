//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1112/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1112<F: Float>(t11475: F, t12056: F, t3262: F, t3469: F, t42413: F, t3264: F, t44555: F, t12210: F, t40664: F, t12206: F, t40282: F, t11621: F, t3275: F, t41791: F, t10610: F, t3465: F, t42934: F) -> (F, F, F, F, F, F, F) {
    let t45058 = 3.0 / 2.0 * t3262 * t12056 * t11475;
    let t45060 = t42413 * t3469 / 4.0;
    let t45066 = 3.0 / 4.0 * t3262 * t44555 * t3264;
    let t45068 = 3.0 / 2.0 * t40664 * t12210;
    let t45070 = 3.0 / 2.0 * t40282 * t12206;
    let t45073 = 45.0 / 32.0 * t3275 * t41791 * t11621;
    let t45078 = 3.0 / 2.0 * t10610 * t3465 * t42934;
    (t45058, t45060, t45066, t45068, t45070, t45073, t45078)
}
