//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1265/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1265<F: Float>(t11531: F, t12098: F, t3275: F, t3262: F, t3465: F, t42959: F, t11336: F, t39263: F, t42863: F, t11325: F, t12428: F, t12951: F, t37282: F) -> (F, F, F, F, F) {
    let t44882 = F::new(5.0) / F::new(8.0) * t3275 * t12098 * t11531;
    let t44885 = F::new(3.0) / F::new(2.0) * t3262 * t3465 * t42959;
    let t44888 = F::new(3.0) * t39263 * t11336 * t42863;
    let t44893 = F::new(5.0) / F::new(16.0) * t3275 * t11325 * t12428;
    let t44897 = F::new(15.0) / F::new(8.0) * t37282 * t12951;
    (t44882, t44885, t44888, t44893, t44897)
}
