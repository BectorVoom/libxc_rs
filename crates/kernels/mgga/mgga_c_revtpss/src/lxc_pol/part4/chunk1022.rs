//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1022/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1022<F: Float>(t13767: F, t13769: F, t2661: F, t13756: F, t13762: F, t13763: F, t13765: F, t1410: F, t9697: F, t9705: F, t9711: F, t9712: F, t9716: F, t9725: F, t9729: F, t550: F, t5658: F) -> (F, F) {
    let t13770 = t13767 * t13769;
    let t13772 = 0.28582678745379824648e-3 * t2661 * t13770;
    let t13773 = 7.0 / 144.0 * t9697 - 0.14291339372689912324e-3 * t9705 + t9711 - 0.60976381323476959249e-3 * t9712 + 0.28582678745379824648e-4 * t9716 + t9725 - t9729 - 0.85748036236139473944e-3 * t1410 * t13756 - t13762 + 0.80031500487063509014e-2 * t13763 + 0.54208002996571016773e-3 * t13765 - t13772;
    let t13774 = t550 * t5658;
    (t13773, t13774)
}
