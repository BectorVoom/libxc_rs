//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 611/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk611<F: Float>(t1131: F, t513: F, t1459: F, t398: F, t1032: F, t1352: F, t1446: F, t997: F, t1008: F, t1418: F, t1347: F, t1413: F, t952: F) -> (F, F, F, F, F, F, F) {
    let t4665 = t513 * t1131;
    let t4667 = t398 * t1459 * t4665;
    let t4670 = t1032 * t1352;
    let t4673 = F::cast_from(0.16006300097412701803e-1_f64) * t997 * t1446;
    let t4675 = F::cast_from(0.34299214494455789578e-2_f64) * t1008 * t1418;
    let t4677 = F::cast_from(0.34299214494455789578e-2_f64) * t1008 * t1347;
    let t4679 = F::cast_from(0.20007875121765877254e-2_f64) * t952 * t1413;
    (t4665, t4667, t4670, t4673, t4675, t4677, t4679)
}
