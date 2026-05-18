//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1192/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1192<F: Float>(t3700: F, t40379: F, t11465: F, t11523: F, t11020: F, t12033: F, t10985: F, t12098: F, t3275: F, t10610: F, t3465: F, t39279: F) -> (F, F, F, F, F) {
    let t41223 = F::new(3.0) / F::new(2.0) * t40379 * t3700;
    let t41225 = F::new(5.0) / F::new(8.0) * t11523 * t11465;
    let t41227 = t11020 * t12033 / F::new(4.0);
    let t41230 = F::new(5.0) / F::new(8.0) * t3275 * t12098 * t10985;
    let t41233 = F::new(3.0) / F::new(2.0) * t10610 * t3465 * t39279;
    (t41223, t41225, t41227, t41230, t41233)
}
