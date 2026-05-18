//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1198/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1198<F: Float>(t108604: F, t108608: F, t108623: F, t108625: F, t108627: F, t108629: F, t114573: F, t114575: F, t114577: F, t114584: F, t114586: F, t96358: F, t96359: F, t98285: F) -> F {
    let t115065 = -F::new(0.17149607247227894789e-3) * t108604 - F::new(0.6098400337114239387e-3) * t108608 - F::new(0.51448821741683684367e-2) * t114573 + F::new(0.51448821741683684367e-1) * t114575 - F::new(0.85748036236139473944e-3) * t114577 - t96358 - t96359 - F::new(0.2168591159877823526e-3) * t98285 + F::new(0.85748036236139473944e-4) * t108623 + F::new(0.30492001685571196935e-2) * t108625 - F::new(0.24009450146119052704e0) * t108627 + F::new(0.48018900292238105409e-1) * t108629 - F::new(0.34299214494455789578e-2) * t114584 - F::new(0.10289764348336736873e0) * t114586;
    t115065
}
