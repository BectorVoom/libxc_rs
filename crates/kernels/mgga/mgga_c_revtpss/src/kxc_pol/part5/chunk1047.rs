//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1047/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1047<F: Float>(t13760: F, t3978: F, t3989: F, t5614: F, t5622: F, t9765: F, t1408: F, t240: F, t1868: F, t4010: F, t1353: F, t2661: F) -> (F, F, F, F) {
    let t13762 = F::cast_from(0.10164000561857065645e-3_f64) * t3978 * t13760;
    let t13763 = t3989 * t5614;
    let t13765 = t9765 * t5622;
    let t13767 = t1408 * t240;
    let t13768 = t4010 * t1868;
    let t13769 = t13768 * t1353;
    let t13770 = t13767 * t13769;
    let t13772 = F::cast_from(0.28582678745379824648e-3_f64) * t2661 * t13770;
    (t13762, t13763, t13765, t13772)
}
