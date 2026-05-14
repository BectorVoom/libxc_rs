//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1213/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1213<F: Float>(t1281: F, t28250: F, t10498: F, t1203: F, t28002: F, t27987: F, t3481: F, t26950: F, t5036: F, t26880: F, t46026: F, t10491: F, t14680: F, t26871: F, t3331: F, t33862: F, t8064: F) -> (F, F, F, F, F, F, F, F) {
    let t97494 = t28250 * t1281;
    let t97499 = 12.0 * t10498 * t28002 * t1203;
    let t97500 = t27987 * t3481;
    let t97501 = t5036 * t26950;
    let t97503 = 6.0 * t46026 * t26880;
    let t97505 = 4.0 * t10491 * t28002;
    let t97507 = 4.0 * t26871 * t14680;
    let t97510 = 24.0 * t33862 * t8064 * t3331;
    (t97494, t97499, t97500, t97501, t97503, t97505, t97507, t97510)
}
