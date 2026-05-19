//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 455/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk455<F: Float>(t1356: F, t1358: F, t1360: F, t1378: F, t1387: F, t1389: F, t1413: F, t1418: F, t1783: F, t2045: F, t2052: F, t2059: F, t2063: F, t2065: F, t2068: F, t246: F, t765: F) -> F {
    let t2073 = F::new(0.571528e-1) * t2045 + t2052 - t2059 - F::cast_from(0.1350520664e0_f64) * t2063 + t1356 - t1358 - t1360 - t1378 + F::cast_from(0.675260332e-1_f64) * t765 * t2065 + F::cast_from(0.1350520664e0_f64) * t765 * t2068 + t1387 + t1389 + t1413 - F::new(0.285764e-1) * t246 * t1783 - t1418;
    t2073
}
