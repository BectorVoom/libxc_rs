//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1334/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1334<F: Float>(t10227: F, t96: F, t100: F, t613: F, t10199: F, t2175: F, t2289: F, t8264: F, t31051: F, t625: F, t31027: F, t31044: F, t2184: F, t4168: F, t31127: F, t571: F) -> (F, F, F, F, F, F, F, F) {
    let t116946 = t96 * t10227;
    let t116957 = t613 * t100;
    let t116968 = 154.0 / 27.0 * t10199 * t2175;
    let t116969 = t2289 * t8264;
    let t116971 = t625 * t31051;
    let t116995 = t31027 * t31044;
    let t117090 = t2184 * t4168;
    let t117095 = t571 * t31127;
    (t116946, t116957, t116968, t116969, t116971, t116995, t117090, t117095)
}
