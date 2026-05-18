//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 769/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk769<F: Float>(t7125: F, t1524: F, t963: F, t6887: F, t970: F, t2271: F, t2816: F, t2747: F, t468: F, t1411: F, t1385: F, t1561: F, t983: F) -> (F, F, F, F, F, F, F, F) {
    let t7126 = F::new(2.0) * t7125;
    let t7127 = t963 * t1524;
    let t7129 = t6887 * t970;
    let t7132 = F::new(0.4726e1) * t2271 * t2816;
    let t7155 = t2747 * t468;
    let t7156 = F::new(0.11696447245269292414e1) * t7155;
    let t7157 = t963 * t1411;
    let t7159 = t963 * t1385;
    let t7217 = t1561 * t983;
    (t7126, t7127, t7129, t7132, t7156, t7157, t7159, t7217)
}
