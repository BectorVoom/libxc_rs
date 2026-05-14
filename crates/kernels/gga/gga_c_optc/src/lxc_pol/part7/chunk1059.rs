//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1059/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1059<F: Float>(t2441: F, t7696: F, t2468: F, t7337: F, t2471: F, t2528: F, t23789: F, t7504: F, t845: F, t2253: F, t7188: F, t23559: F, t24003: F, t24006: F, t24008: F, t24014: F, t2549: F, t7175: F, t7325: F, t8280: F, t8387: F, t914: F, t999: F) -> (F, F, F, F, F) {
    let t24017 = 0.14035736153892489771e2 * t2441 * t7696;
    let t24019 = 0.35089340384731224426e1 * t7337 * t2468;
    let t24021 = 1.0 / t2471 / t2528;
    let t24025 = 0.12304676425209353917e5 * t845 * t24021 * t23789 * t7504;
    let t24026 = t7188 * t2253;
    let t24032 = -2.0 / 9.0 * t24003 - 8.0 / 27.0 * t24006 - 4.0 / 3.0 * t24008 + 8.0 * t999 * t914 * t2549 * t23559 + 176.0 / 27.0 * t24014 - t24017 - t24019 + t24025 - 200.0 / 3.0 * t24026 - 400.0 / 27.0 * t8280 * t7175 - 400.0 / 9.0 * t7325 * t8387;
    (t24017, t24019, t24021, t24025, t24032)
}
