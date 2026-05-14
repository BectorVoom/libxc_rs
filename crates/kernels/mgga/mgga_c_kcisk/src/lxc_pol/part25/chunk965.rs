//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 965/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk965<F: Float>(t17116: F, t1869: F, t4581: F, t6698: F, t1799: F, t4644: F, t6697: F, t1800: F, t2441: F, t5063: F, t1899: F, t5062: F, t11204: F, t11652: F, t16562: F, t17005: F, t17074: F, t17077: F, t17078: F, t17084: F, t17087: F, t17093: F, t17096: F, t17100: F, t17104: F, t17109: F, t17114: F, t2470: F, t4823: F, t4827: F, t671: F) -> (F, F, F, F, F, F) {
    let t17117 = t1869 * t17116;
    let t17119 = t4581 * t6698;
    let t17120 = t1799 * t17119;
    let t17122 = t6697 * t4644;
    let t17123 = t1800 * t17122;
    let t17124 = t1799 * t17123;
    let t17126 = t2441 * t5063;
    let t17127 = t1899 * t17126;
    let t17128 = t5062 * t17127;
    let t17129 = t1869 * t17128;
    let t17131 = -0.33163888888888888888e-2 * t17074 + t17077 + 0.74498e-1 * t17078 * t4827 - 0.193e0 * t11204 * t2470 + 0.27636574074074074073e-2 * t17084 + t17087 + t16562 * t671 - 0.223494e0 * t4823 * t17005 - 0.44218518518518518517e-2 * t17093 + 0.16581944444444444444e-2 * t17096 - 0.44218518518518518517e-2 * t17100 - 0.73697530864197530862e-2 * t17104 + 0.16581944444444444444e-2 * t11652 + 0.88437037037037037034e-2 * t17109 - 0.16581944444444444444e-2 * t17114 - 0.11054629629629629629e-2 * t17117 - 0.88437037037037037034e-2 * t17120 + 0.88437037037037037034e-2 * t17124 + 0.33163888888888888888e-2 * t17129;
    (t17117, t17120, t17124, t17126, t17129, t17131)
}
