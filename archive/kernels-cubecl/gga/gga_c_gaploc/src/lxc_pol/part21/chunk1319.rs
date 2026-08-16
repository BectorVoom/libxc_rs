//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1319/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1319<F: Float>(t34488: F, t6895: F, t9263: F, t993: F, t34371: F, t6963: F, t6964: F, t10354: F, t20003: F, t1402: F, t1429: F, t3380: F) -> (F, F, F, F, F) {
    let t34489 = F::cast_from(0.76685851907841499352e0_f64) * t34488;
    let t34491 = t9263 * t993 * t6895;
    let t34492 = F::cast_from(0.38342925953920749676e0_f64) * t34491;
    let t34498 = F::cast_from(0.14300195980740170668e1_f64) * t6963 * t6964 * t34371;
    let t34500 = F::cast_from(0.23005755572352449806e2_f64) * t20003 * t10354;
    let t34502 = t1429 * t1402 * t3380;
    (t34489, t34492, t34498, t34500, t34502)
}
