//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 975/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk975<F: Float>(t3555: F, t633: F, t3402: F, t4934: F, t1620: F, t3406: F, t5137: F, t639: F, t3554: F, t582: F, t211: F, t2601: F, t2722: F) -> (F, F, F, F, F) {
    let t11018 = F::new(2.0) / F::new(15.0) * t633 * t3555;
    let t11019 = t4934 * t3402;
    let t11020 = t1620 * t11019;
    let t11021 = F::new(32.0) / F::new(135.0) * t11020;
    let t11022 = t5137 * t3406;
    let t11023 = t639 * t11022;
    let t11024 = F::new(16.0) / F::new(135.0) * t11023;
    let t11025 = t582 * t3554;
    let t11026 = t211 * t11025;
    let t11027 = F::new(4.0) / F::new(45.0) * t11026;
    let t11028 = t2601 * t2722;
    (t11018, t11021, t11024, t11027, t11028)
}
