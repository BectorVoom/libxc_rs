//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 970/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk970<F: Float>(t235: F, t9731: F, t1389: F, t3964: F, t2735: F, t546: F, t1353: F, t1412: F, t808: F, t1369: F, t2699: F, t1372: F) -> (F, F, F, F, F, F) {
    let t9732 = t235 * t9731;
    let t9735 = F::cast_from(0.81322168495418382223e-4_f64) * t3964 * t9732 * t1389;
    let t9736 = t2735 * t546;
    let t9737 = t1412 * t1353;
    let t9738 = t808 * t9737;
    let t9739 = t9736 * t9738;
    let t9741 = t2699 * t1369;
    let t9742 = t9741 * t1372;
    (t9732, t9735, t9736, t9739, t9741, t9742)
}
