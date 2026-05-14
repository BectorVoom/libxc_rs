//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 749/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk749<F: Float>(t7116: F, t1543: F, t288: F, t910: F, t97: F, t2483: F, t457: F, t41: F, t1524: F, t963: F, t6887: F, t970: F, t2271: F, t2816: F, t1376: F, t955: F) -> (F, F, F, F, F, F, F) {
    let t7117 = 3.0 * t7116;
    let t7118 = t1543 * t288;
    let t7120 = t97 * t7118 * t910;
    let t7121 = 6.0 * t7120;
    let t7124 = t2483 * t457;
    let t7125 = t41 * t7124;
    let t7126 = 2.0 * t7125;
    let t7127 = t963 * t1524;
    let t7128 = 0.11696447245269292414e1 * t7127;
    let t7129 = t6887 * t970;
    let t7132 = 0.4726e1 * t2271 * t2816;
    let t7133 = t1376 * t955;
    (t7117, t7121, t7126, t7128, t7129, t7132, t7133)
}
