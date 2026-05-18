//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 546/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk546<F: Float>(t124: F, t3615: F, t1149: F, t329: F, t1152: F, t1140: F, t1156: F, t1133: F, t1117: F, t1137: F, t1121: F, t107: F, t2607: F, t2690: F, t4: F) -> (F, F, F, F, F, F, F, F) {
    let t3616 = t124 * t3615;
    let t3621 = t329 * t1149;
    let t3622 = t3621 * t1152;
    let t3624 = t1140 * t1156;
    let t3634 = t1140 * t1133;
    let t3636 = t1137 * t1117;
    let t3638 = t1140 * t1121;
    let t3644 = -F::new(0.12962962962962962963e0) * t4 * t2607 * t107 - F::new(0.40124259259259259261e-1) * t2690;
    (t3616, t3621, t3622, t3624, t3634, t3636, t3638, t3644)
}
