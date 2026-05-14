//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 850/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk850<F: Float>(t3530: F, t5283: F, t587: F, t2598: F, t7527: F, t3535: F, t7136: F, t5312: F, t2635: F, t2784: F, t1885: F, t1820: F, t3455: F, t582: F, t185: F, t2705: F, t954: F) -> (F, F, F, F, F, F, F) {
    let t10472 = t5283 * t3530;
    let t10473 = t587 * t10472;
    let t10474 = 8.0 / 81.0 * t10473;
    let t10476 = 8.0 / 15.0 * t7527 * t2598;
    let t10478 = 8.0 / 15.0 * t7136 * t3535;
    let t10480 = 8.0 / 15.0 * t5312 * t3535;
    let t10481 = t2635 * t2784;
    let t10482 = t1885 * t10481;
    let t10484 = 8.0 / 15.0 * t1820 * t10482;
    let t10485 = t582 * t3455;
    let t10486 = t185 * t10485;
    let t10487 = 8.0 / 45.0 * t10486;
    let t10488 = t2705 * t954;
    (t10474, t10476, t10478, t10480, t10484, t10487, t10488)
}
