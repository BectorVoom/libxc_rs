//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 647/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk647<F: Float>(t128: F, t1864: F, t118: F, t1986: F, t7408: F, t1737: F, t645: F, t4044: F, t2344: F, t8659: F, t2329: F, t8365: F, t209: F, t605: F, t615: F, t236: F) -> (F, F, F, F, F, F, F) {
    let t10048 = t128 * t1864;
    let t10049 = t118 * t10048;
    let t10050 = t1986 * t10049;
    let t10051 = t7408 * t10050;
    let t10053 = t645 * t1737;
    let t10054 = t4044 * t10053;
    let t10056 = t8659 * t2344;
    let t10058 = t8365 * t2329;
    let t10064 = t615 * t605 * t209;
    let t10065 = t236 * t10064;
    (t10050, t10051, t10053, t10054, t10056, t10058, t10065)
}
