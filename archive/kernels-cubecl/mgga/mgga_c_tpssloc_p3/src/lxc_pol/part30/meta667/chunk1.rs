//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2094/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2094<F: Float>(t1354: F, t91285: F, t26298: F, t80958: F, t22779: F, t26319: F, t1358: F, t26248: F, t3862: F, t7715: F, t22705: F, t22852: F, t236: F, t5286: F, t550: F) -> (F, F, F, F, F, F) {
    let t91286 = t91285 * t1354;
    let t91287 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t91286;
    let t91290 = t80958 * t26298;
    let t91300 = t22779 * t26319;
    let t91301 = F::cast_from(0.56521858531796547196e-2_f64) * t91300;
    let t91303 = t26248 * t1358;
    let t91304 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t91303;
    let t91305 = t7715 * t3862;
    let t91310 = t22852 * t22705 * t236 * t5286 * t550;
    (t91287, t91290, t91301, t91304, t91305, t91310)
}
