//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1252/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1252<F: Float>(t3245: F, t3270: F, t39030: F, t3269: F, t1115: F, t3016: F, t10667: F, t11342: F, t42389: F, t3262: F, t3465: F, t43984: F) -> (F, F, F, F) {
    let t44568 = t3270 * t39030 * t3245;
    let t44570 = t3269 * t44568 / F::cast_from(2.0_f64);
    let t44572 = t3270 * t1115 * t3016;
    let t44574 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t10667 * t44572;
    let t44576 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t42389 * t11342;
    let t44579 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3262 * t3465 * t43984;
    (t44570, t44574, t44576, t44579)
}
