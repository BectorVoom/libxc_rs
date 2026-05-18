//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1015/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1015<F: Float>(t12203: F, t3269: F, t11497: F, t3465: F, t3262: F, t1115: F, t3270: F, t910: F, t10667: F, t11342: F, t11506: F, t11509: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12204 = t3269 * t12203;
    let t12205 = F::new(5.0) / F::new(16.0) * t12204;
    let t12206 = t3465 * t11497;
    let t12207 = t3262 * t12206;
    let t12208 = F::new(3.0) / F::new(4.0) * t12207;
    let t12210 = t3270 * t1115 * t910;
    let t12211 = t10667 * t12210;
    let t12212 = F::new(3.0) / F::new(4.0) * t12211;
    let t12213 = t11506 * t11342;
    let t12214 = F::new(3.0) / F::new(4.0) * t12213;
    let t12215 = t3465 * t11509;
    (t12204, t12205, t12206, t12207, t12208, t12210, t12211, t12212, t12213, t12214, t12215)
}
