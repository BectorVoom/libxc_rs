//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1284/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1284<F: Float>(t11475: F, t12056: F, t3262: F, t3469: F, t42413: F, t3264: F, t44555: F, t12210: F, t40664: F, t12206: F, t40282: F, t11621: F, t3275: F, t41791: F) -> (F, F, F, F, F, F) {
    let t45058 = F::new(3.0) / F::new(2.0) * t3262 * t12056 * t11475;
    let t45060 = t42413 * t3469 / F::new(4.0);
    let t45066 = F::new(3.0) / F::new(4.0) * t3262 * t44555 * t3264;
    let t45068 = F::new(3.0) / F::new(2.0) * t40664 * t12210;
    let t45070 = F::new(3.0) / F::new(2.0) * t40282 * t12206;
    let t45073 = F::new(45.0) / F::new(32.0) * t3275 * t41791 * t11621;
    (t45058, t45060, t45066, t45068, t45070, t45073)
}
