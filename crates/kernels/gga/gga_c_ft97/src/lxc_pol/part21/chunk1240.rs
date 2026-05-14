//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1240/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1240<F: Float>(t100678: F, t1013: F, t105124: F, t105127: F, t115513: F, t118876: F, t118954: F, t118970: F, t118976: F, t1557: F, t23705: F, t23715: F, t23869: F, t3188: F, t40081: F, t423: F, t4431: F, t538: F, t554: F, t5570: F, t5838: F, t94429: F, t94524: F) -> (F,) {
    let t118986 = 0.44452000728395061731e-1 * t5838 * t115513 - 0.55565000910493827163e-2 * t118954 - 0.33339000546296296298e-1 * t23715 * t5570 * t423 * t4431 * t538 + 0.33339000546296296298e-1 * t23705 * t5570 * t423 * t4431 * t554 - 0.48327307107230638238e1 * t23869 * t118876 - 0.90613700826057446696e0 * t40081 * t118970 - 0.66678001092592592595e-1 * t105124 - 0.66678001092592592595e-1 * t105127 - 0.1611184118048991131e0 * t94429 * t118976 + 0.1611184118048991131e0 * t94524 * t118976 + 0.8890400145679012346e-1 * t23705 * t100678 * t1013 * t1557 * t3188;
    (t118986,)
}
