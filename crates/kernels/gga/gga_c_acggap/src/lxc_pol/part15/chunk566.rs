//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 566/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk566<F: Float>(t1140: F, t1511: F, t1137: F, t1494: F, t1498: F, t1032: F, t1352: F, t1446: F, t997: F, t1008: F, t1418: F, t1347: F) -> (F, F, F, F, F, F, F) {
    let t4649 = F::new(7.0) / F::new(144.0) * t1140 * t1511;
    let t4651 = F::new(7.0) / F::new(72.0) * t1137 * t1494;
    let t4653 = F::new(7.0) / F::new(72.0) * t1137 * t1498;
    let t4670 = t1032 * t1352;
    let t4673 = F::cast_from(0.16006300097412701803e-1_f64) * t997 * t1446;
    let t4675 = F::cast_from(0.34299214494455789578e-2_f64) * t1008 * t1418;
    let t4677 = F::cast_from(0.34299214494455789578e-2_f64) * t1008 * t1347;
    (t4649, t4651, t4653, t4670, t4673, t4675, t4677)
}
