//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 708/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk708<F: Float>(t1291: F, t4540: F, t1: F, t2057: F, t793: F, t2062: F, t700: F, t762: F, t1597: F, t1383: F, t528: F, t35: F, t413: F, t536: F, t1477: F, t6: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4541 = t1291 * t4540;
    let t4542 = 6.0 * t4541;
    let t4544 = t793 * t2057 * t1;
    let t4545 = t4544 * t2062;
    let t4550 = 0.50257692321302641125e0 * t762 * t700;
    let t4554 = t1597 * t700;
    let t4557 = 0.25128846160651320563e0 * t528 * t1383;
    let t4560 = t35 * t413;
    let t4561 = 24.0 * t4560;
    let t4568 = t536 * t1383;
    let t4573 = t6 * t1477;
    (t4542, t4545, t4550, t4554, t4557, t4560, t4561, t4568, t4573)
}
