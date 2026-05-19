//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1209/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1209<F: Float>(t94568: F, t94570: F, t94534: F, t94537: F, t94540: F, t94542: F, t94546: F, t94548: F, t94552: F, t94554: F, t94557: F, t94559: F, t94561: F, t94565: F) -> F {
    let t96358 = F::cast_from(0.45178982497454656792e-6_f64) * t94568;
    let t96359 = F::cast_from(0.28900264064772933812e-2_f64) * t94570;
    let t96360 = -F::cast_from(0.17149607247227894789e-2_f64) * t94534 + F::cast_from(0.30492001685571196935e-4_f64) * t94537 - F::cast_from(0.2168591159877823526e-3_f64) * t94540 - F::cast_from(0.6098400337114239387e-3_f64) * t94542 - F::cast_from(0.27210710165601593065e0_f64) * t94546 + F::cast_from(0.48018900292238105409e-1_f64) * t94548 - F::cast_from(0.17149607247227894789e-3_f64) * t94552 - F::cast_from(0.91464571985215438874e-3_f64) * t94554 + F::cast_from(0.85748036236139473944e-4_f64) * t94557 - F::cast_from(0.24009450146119052704e0_f64) * t94559 + F::cast_from(0.30492001685571196935e-2_f64) * t94561 - F::cast_from(0.54214778996945588151e-4_f64) * t94565 - t96358 - t96359;
    t96360
}
