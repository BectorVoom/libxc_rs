//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 745/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk745<F: Float>(t404: F, t4536: F, t389: F, t4510: F, t1291: F, t1: F, t2057: F, t793: F, t2062: F, t700: F, t762: F, t1597: F) -> (F, F, F, F, F) {
    let t4537 = t4536 * t404;
    let t4538 = t389 * t4537;
    let t4539 = F::new(1.0) * t4538;
    let t4540 = t4510 * t404;
    let t4541 = t1291 * t4540;
    let t4542 = F::new(6.0) * t4541;
    let t4544 = t793 * t2057 * t1;
    let t4545 = t4544 * t2062;
    let t4550 = F::cast_from(0.50257692321302641125e0_f64) * t762 * t700;
    let t4554 = t1597 * t700;
    (t4539, t4542, t4545, t4550, t4554)
}
