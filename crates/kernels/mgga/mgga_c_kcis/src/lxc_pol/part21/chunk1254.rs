//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1254/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1254<F: Float>(t27747: F, t27750: F, t27753: F, t27756: F, t10488: F, t8081: F, t1141: F, t27985: F, t1203: F, t10498: F, t3481: F, t8064: F) -> (F, F, F, F, F, F, F) {
    let t95278 = t27747 / F::cast_from(8.0_f64);
    let t95279 = t27750 / F::cast_from(8.0_f64);
    let t95280 = t27753 / F::cast_from(8.0_f64);
    let t95281 = t27756 / F::cast_from(8.0_f64);
    let t95285 = t10488 * t8081;
    let t95286 = t27985 * t1141;
    let t95288 = F::cast_from(2.0_f64) * t95286 * t1203;
    let t95291 = F::cast_from(6.0_f64) * t10498 * t8064 * t3481;
    (t95278, t95279, t95280, t95281, t95285, t95288, t95291)
}
