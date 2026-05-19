//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 473/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk473<F: Float>(t1960: F, t551: F, t553: F, t1369: F, t163: F, t148: F, t1371: F, t550: F, t283: F, t799: F) -> (F, F, F, F, F) {
    let t1962 = t1960 * t551 * t553;
    let t1964 = t1369 * t163;
    let t1966 = F::cast_from(0.31505407223141117834e-1_f64) * t148 * t1964;
    let t1969 = F::cast_from(0.39507780657818961764e-2_f64) * t550 * t1371 * t553;
    let t1971 = t799 * t283;
    (t1962, t1964, t1966, t1969, t1971)
}
