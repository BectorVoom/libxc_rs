//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1708/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1708<F: Float>(t17539: F, t5296: F, t1042: F, t3172: F, t5286: F, t1247: F, t3707: F, t5292: F, t12268: F, t3617: F, t15936: F, t3708: F, t5265: F) -> (F, F, F, F, F, F) {
    let t17540 = t5296 * t17539;
    let t17541 = t1042 * t17540;
    let t17544 = t3172 * t5286;
    let t17546 = F::cast_from(0.28582678745379824648e-3_f64) * t1247 * t17544;
    let t17547 = t3707 * t5292;
    let t17550 = t3617 * t12268;
    let t17551 = t17550 * t15936;
    let t17552 = t1042 * t17551;
    let t17556 = F::cast_from(0.28582678745379824648e-3_f64) * t3708 * t5265;
    (t17541, t17544, t17546, t17547, t17552, t17556)
}
