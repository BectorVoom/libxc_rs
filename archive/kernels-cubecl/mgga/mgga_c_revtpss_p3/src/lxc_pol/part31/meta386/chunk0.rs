//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1424/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1424<F: Float>(t3172: F, t4868: F, t1041: F, t3168: F, t4878: F, t11150: F, t3181: F, t11144: F, t11852: F, t3124: F, t4820: F, t1655: F, t697: F) -> (F, F, F, F, F, F, F) {
    let t16163 = t3172 * t4868;
    let t16165 = F::cast_from(0.28582678745379824648e-3_f64) * t1041 * t16163;
    let t16190 = t4878 * t3168;
    let t16199 = t3181 * t11150;
    let t16208 = t11852 * t11144;
    let t16218 = F::cast_from(0.28582678745379824648e-3_f64) * t3124 * t4820;
    let t16219 = t697 * t1655;
    (t16163, t16165, t16190, t16199, t16208, t16218, t16219)
}
