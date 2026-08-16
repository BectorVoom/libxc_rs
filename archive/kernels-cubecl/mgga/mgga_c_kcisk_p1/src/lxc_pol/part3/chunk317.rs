//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 317/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk317<F: Float>(t1512: F, t499: F, t493: F, t1286: F, t381: F, t498: F, t1484: F, t1490: F, t1494: F, t1498: F, t1502: F, t1507: F) -> (F, F, F, F, F, F) {
    let t1513 = t1512 * t499;
    let t1514 = t493 * t1513;
    let t1516 = t381 * t1286;
    let t1517 = t498 * t1516;
    let t1518 = t493 * t1517;
    let t1520 = t1484 / F::cast_from(16.0_f64) - t1490 / F::cast_from(16.0_f64) - t1494 / F::cast_from(6.0_f64) + t1498 / F::cast_from(24.0_f64) - t1502 / F::cast_from(256.0_f64) + t1507 / F::cast_from(256.0_f64) + t1514 / F::cast_from(48.0_f64) - t1518 / F::cast_from(192.0_f64);
    (t1513, t1514, t1516, t1517, t1518, t1520)
}
