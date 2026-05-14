//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 872/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk872<F: Float>(t4589: F, t1852: F, t20044: F, t920: F, t4417: F, t4551: F, t11472: F, t11854: F, t16246: F, t1871: F, t1901: F, t20045: F, t20177: F, t20218: F, t20268: F, t20279: F, t20395: F, t2992: F, t3238: F, t39120: F, t4436: F, t446: F, t447: F, t4495: F, t452: F, t4572: F, t488: F, t74899: F, t925: F, t942: F, t986: F) -> (F, F, F, F) {
    let t85315 = t4589 * t4589;
    let t85316 = t1852 * t85315;
    let t85320 = t20044 * t920;
    let t85325 = t4417 * t4551;
    let t85380 = -4.0 * t446 * t1871 * t488 * t4436 * t4589 - 4.0 / 9.0 * t446 * t447 * t986 * t20045 + 4.0 * t446 * t452 * t16246 * t4572 - 8.0 * t446 * t1871 * t3238 * t20177 - 8.0 / 3.0 * t74899 + 8.0 / 3.0 * t1901 * t39120 * t20177 * t925 - 8.0 / 3.0 * t1901 * t11854 * t20268 * t925 - 8.0 / 3.0 * t1901 * t11472 * t2992 * t20218 + 4.0 * t446 * t452 * t3238 * t20279 + 2.0 * t446 * t452 * t488 * t4495 * t4589 + 4.0 / 3.0 * t446 * t452 * t488 * t942 * t20395;
    (t85316, t85320, t85325, t85380)
}
