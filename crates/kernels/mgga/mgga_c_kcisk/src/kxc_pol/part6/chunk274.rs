//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 274/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk274<F: Float>(t1173: F, t420: F, t1161: F, t1346: F, t429: F, t431: F, t446: F, t301: F, t41: F) -> (F, F, F, F, F) {
    let t1355 = t1173 * t420;
    let t1359 = F::new(0.41275e-2) * t1161;
    let t1369 = F::cast_from(0.11955719325063177623e-1_f64) * t1346;
    let t1374 = F::new(0.3513e-2) * t429 * t446 * t431;
    let t1375 = t41 * t301;
    (t1355, t1359, t1369, t1374, t1375)
}
