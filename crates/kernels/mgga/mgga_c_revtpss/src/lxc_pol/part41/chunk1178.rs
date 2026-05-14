//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1178/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1178<F: Float>(t19716: F, t4893: F, t3117: F, t19450: F, t4900: F, t11661: F, t19501: F, t3092: F, t1047: F, t1063: F, t12013: F, t16067: F, t16089: F, t19688: F, t19693: F, t19697: F, t19702: F, t19707: F, t3127: F, t4803: F, t4808: F, t4834: F, t4892: F, t4899: F, t6308: F) -> (F,) {
    let t19717 = t4893 * t19716;
    let t19718 = t3117 * t19717;
    let t19721 = t19450 * t4900;
    let t19722 = t3117 * t19721;
    let t19725 = t19501 * t11661;
    let t19726 = t3092 * t19725;
    let t19729 = 0.23818898954483187207e-3 * t1063 * t19688 - 0.23818898954483187207e-3 * t3127 * t19693 + 0.21437009059034868486e-3 * t19697 * t1047 - 0.14291339372689912324e-3 * t3127 * t19702 + 0.57165357490759649296e-3 * t16089 * t19707 - 0.22866142996303859718e-2 * t12013 * t6308 - 0.57165357490759649296e-3 * t4834 * t4803 + 0.47637797908966374413e-3 * t4834 * t4808 - 0.42874018118069736972e-3 * t4899 * t19718 + 0.21437009059034868486e-3 * t16067 * t19722 + 0.28582678745379824648e-3 * t4892 * t19726;
    (t19729,)
}
