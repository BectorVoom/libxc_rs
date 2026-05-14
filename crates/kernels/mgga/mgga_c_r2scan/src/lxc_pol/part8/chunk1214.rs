//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1214/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1214<F: Float>(t20758: F, t2627: F, t2631: F, t6240: F, t20818: F, t20820: F, t2604: F, t2183: F, t7983: F, t20422: F, t20424: F, t8123: F, t20434: F, t8129: F, t2531: F, t6212: F) -> (F, F, F, F, F, F, F) {
    let t25664 = t20758 * t2627;
    let t25665 = 0.38415120233790484326e1 * t25664;
    let t25666 = t6240 * t2631;
    let t25667 = 0.64025200389650807209e0 * t25666;
    let t25715 = t20818 * t2604 * t20820;
    let t25720 = t2183 * t7983;
    let t25726 = t20422 * t8123 * t20424;
    let t25728 = t20434 * t8129;
    let t25729 = 0.19043987679069580388e-1 * t25728;
    let t25737 = t6212 * t2531;
    (t25665, t25667, t25715, t25720, t25726, t25729, t25737)
}
