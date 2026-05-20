//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1434/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1434<F: Float>(t3368: F, t5277: F, t1042: F, t3704: F, t5274: F, t1774: F, t3588: F, t1250: F, t3720: F, t1285: F, t17395: F, t1032: F, t5216: F) -> (F, F, F, F, F, F) {
    let t17588 = t5277 * t3368;
    let t17589 = t1042 * t17588;
    let t17593 = F::cast_from(0.28582678745379824648e-3_f64) * t5274 * t3704;
    let t17600 = t1774 * t3588;
    let t17601 = t17600 * t1250;
    let t17602 = t3720 * t17601;
    let t17605 = t1285 * t17395;
    let t17608 = t5216 * t1032;
    (t17589, t17593, t17600, t17602, t17605, t17608)
}
