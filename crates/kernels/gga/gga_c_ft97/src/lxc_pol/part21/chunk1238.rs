//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1238/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1238<F: Float>(t104735: F, t25713: F, t93048: F, t100678: F, t1008: F, t105038: F, t105044: F, t105056: F, t105058: F, t115702: F, t1557: F, t1570: F, t16786: F, t23715: F, t26692: F, t3188: F, t4675: F, t5530: F, t93169: F, t94530: F, t94535: F) -> (F,) {
    let t118902 = t93048 * t104735 * t25713;
    let t118919 = -0.24163653553615319119e1 * t16786 * t5530 + 0.13335600218518518519e0 * t23715 * t93169 * t1008 * t1570 * t3188 + 0.1611184118048991131e0 * t94530 * t118902 - 0.1611184118048991131e0 * t94535 * t118902 - 0.8890400145679012346e-1 * t23715 * t100678 * t1008 * t1557 * t3188 + 0.29634667152263374488e-1 * t105038 - 0.22226000364197530866e-1 * t105044 + t105056 + 0.7408666788065843622e-2 * t105058 + 0.24163653553615319119e1 * t4675 * t5530 + 0.33339000546296296298e-1 * t26692 * t115702;
    (t118919,)
}
