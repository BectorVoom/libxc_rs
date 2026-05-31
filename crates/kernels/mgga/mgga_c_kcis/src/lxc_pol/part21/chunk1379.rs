//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1379/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1379<F: Float>(t1281: F, t28250: F, t10498: F, t1203: F, t28002: F, t27987: F, t3481: F, t26950: F, t5036: F, t26880: F, t46026: F, t10491: F) -> (F, F, F, F, F, F) {
    let t97494 = t28250 * t1281;
    let t97499 = F::cast_from(12.0_f64) * t10498 * t28002 * t1203;
    let t97500 = t27987 * t3481;
    let t97501 = t5036 * t26950;
    let t97503 = F::cast_from(6.0_f64) * t46026 * t26880;
    let t97505 = F::cast_from(4.0_f64) * t10491 * t28002;
    (t97494, t97499, t97500, t97501, t97503, t97505)
}
