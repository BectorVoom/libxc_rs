//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 719/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk719<F: Float>(t2484: F, t406: F, t410: F, t1416: F, t899: F, t1419: F, t2483: F, t457: F, t41: F, t1524: F, t963: F, t6887: F, t970: F, t2271: F, t2816: F, t2747: F, t468: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7094 = t406 * t2484;
    let t7095 = 8.0 * t7094;
    let t7096 = t410 * t2484;
    let t7097 = 8.0 * t7096;
    let t7109 = t1416 * t899;
    let t7111 = t1419 * t899;
    let t7124 = t2483 * t457;
    let t7125 = t41 * t7124;
    let t7126 = 2.0 * t7125;
    let t7127 = t963 * t1524;
    let t7129 = t6887 * t970;
    let t7132 = 0.4726e1 * t2271 * t2816;
    let t7155 = t2747 * t468;
    (t7095, t7097, t7109, t7111, t7126, t7127, t7129, t7132, t7155)
}
