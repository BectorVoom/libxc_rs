//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1059/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1059<F: Float>(t108587: F, t108590: F, t108592: F, t108601: F, t114564: F, t114566: F, t96323: F, t96326: F, t96341: F, t96342: F, t98218: F, t98220: F, t98224: F, t98260: F, t108604: F, t108608: F, t108623: F, t108625: F, t108627: F, t108629: F, t114573: F, t114575: F, t114577: F, t114584: F, t114586: F, t96358: F, t96359: F, t98285: F) -> (F, F) {
    let t115052 = -t96323 - 0.3658582879408617555e-2 * t98218 + 0.34299214494455789577e-3 * t108587 - 0.54214778996945588151e-4 * t98220 - 0.24009450146119052704e-1 * t108590 + 0.12004725073059526352e-1 * t108592 - 0.68026775414003982662e-1 * t98224 + t96326 - 0.85748036236139473944e-3 * t114564 + 0.51448821741683684367e-2 * t114566 - 35.0 / 36.0 * t98260 - t96341 + t96342 + 0.85748036236139473944e-4 * t108601;
    let t115065 = -0.17149607247227894789e-3 * t108604 - 0.6098400337114239387e-3 * t108608 - 0.51448821741683684367e-2 * t114573 + 0.51448821741683684367e-1 * t114575 - 0.85748036236139473944e-3 * t114577 - t96358 - t96359 - 0.2168591159877823526e-3 * t98285 + 0.85748036236139473944e-4 * t108623 + 0.30492001685571196935e-2 * t108625 - 0.24009450146119052704e0 * t108627 + 0.48018900292238105409e-1 * t108629 - 0.34299214494455789578e-2 * t114584 - 0.10289764348336736873e0 * t114586;
    (t115052, t115065)
}
