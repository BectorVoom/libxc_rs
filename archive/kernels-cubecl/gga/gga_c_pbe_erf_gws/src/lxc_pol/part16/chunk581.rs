//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 581/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk581<F: Float>(t2554: F, t418: F, t1821: F, t587: F, t1661: F, t197: F, t1663: F, t950: F, t1017: F, t572: F, t1827: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2555 = t2554 * t418;
    let t2556 = t1821 * t2555;
    let t2558 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t2556;
    let t2559 = t1661 * t197;
    let t2560 = t1663 * t950;
    let t2561 = t2560 * t418;
    let t2562 = t2559 * t2561;
    let t2564 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t587 * t2562;
    let t2565 = t1017 * t572;
    let t2566 = t2565 * t418;
    let t2567 = t1827 * t2566;
    let t2569 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t587 * t2567;
    (t2555, t2556, t2558, t2559, t2560, t2561, t2562, t2564, t2565, t2566, t2567, t2569)
}
