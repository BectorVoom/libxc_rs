//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 281/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk281<F: Float>(t343: F, t55: F, t97: F, t360: F, t4: F, t44: F, t375: F, t79: F, t1112: F, t1114: F, t1116: F, t1144: F, t1146: F, t1148: F, t367: F, t374: F) -> (F, F, F, F, F, F) {
    let t1163 = t343 * t97 * t55;
    let t1165 = 0.24415406715670879921e-3 * t360 * t1163;
    let t1166 = t44 * t4;
    let t1167 = t79 * t375;
    let t1169 = 0.10843580882781524214e-1 * t1166 * t1167;
    let t1176 = -0.57538888888888888889e0 * t1112 + 0.11507777777777777778e1 * t1114 + 0.40256666666666666667e0 * t1116 + 0.366775e-1 * t1144 + 0.73355e-1 * t1146 + 0.137975e0 * t1148;
    let t1178 = t367 * t1176 * t374;
    (t1163, t1165, t1167, t1169, t1176, t1178)
}
