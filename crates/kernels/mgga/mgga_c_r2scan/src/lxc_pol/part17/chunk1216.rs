//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1216/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1216<F: Float>(t3262: F, t3574: F, t41202: F, t12045: F, t40282: F, t12060: F, t40713: F, t3275: F, t3465: F, t42940: F, t39030: F, t40630: F, t43771: F) -> (F, F, F, F, F) {
    let t44147 = F::new(3.0) / F::new(2.0) * t3262 * t41202 * t3574;
    let t44150 = F::new(3.0) / F::new(2.0) * t40282 * t12045;
    let t44152 = F::new(5.0) / F::new(8.0) * t40713 * t12060;
    let t44155 = t3275 * t3465 * t42940 / F::new(2.0);
    let t44158 = F::new(3.0) * t40630 * t39030 * t43771;
    (t44147, t44150, t44152, t44155, t44158)
}
