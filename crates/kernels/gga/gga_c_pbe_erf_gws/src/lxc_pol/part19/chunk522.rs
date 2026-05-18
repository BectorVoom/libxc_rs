//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 522/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk522<F: Float>(t2559: F, t2561: F, t587: F, t1017: F, t572: F, t418: F, t1827: F, t1022: F, t626: F, t422: F, t1809: F, t1620: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2562 = t2559 * t2561;
    let t2564 = F::new(4.0) / F::new(27.0) * t587 * t2562;
    let t2565 = t1017 * t572;
    let t2566 = t2565 * t418;
    let t2567 = t1827 * t2566;
    let t2569 = F::new(4.0) / F::new(45.0) * t587 * t2567;
    let t2570 = t1022 * t626;
    let t2571 = t2570 * t422;
    let t2572 = t1809 * t2571;
    let t2574 = F::new(8.0) / F::new(45.0) * t1620 * t2572;
    (t2562, t2564, t2566, t2567, t2569, t2570, t2571, t2572, t2574)
}
