//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1591;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1592;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta421<F: Float>(t12116: F, t4891: F, t3172: F, t4874: F, t3127: F, t4802: F, t1063: F, t4807: F, t11723: F, t11728: F, t11730: F, t11732: F, t11737: F, t11745: F, t3106: F, t4803: F, t4808: F, t4896: F, t3153: F, t4866: F, t4894: F, t3117: F, t3133: F, t3154: F, t4893: F, t13396: F, t4801: F, t1042: F, t11922: F, t4911: F, t3115: F, t15158: F, t4915: F, t1469: F, t3075: F, t4872: F, t1011: F, t11753: F, t11756: F, t11763: F, t11866: F, t3241: F, t4892: F, t4907: F, t4916: F, t4920: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15769, t15772, t15775, t15779) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1591::<F>(t12116, t4891, t3172, t4874, t3127, t4802, t1063, t4807, t11723, t11728, t11730, t11732, t11737, t11745, t3106, t4803, t4808, t4896);
        let (t15780, t15782, t15787, t15791, t15794) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1592::<F>(t3153, t4866, t4894, t3117, t3133, t3154, t4893, t13396, t4801, t1042, t11922, t4911);
        let (t15811, t15814) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1593::<F>(t15794, t3115, t15158, t4915, t1469, t3075, t4872, t1042, t1011, t1063, t11753, t11756, t11763, t11866, t15782, t15787, t15791, t3127, t3241, t4892, t4907, t4916, t4920);
    (t15769, t15772, t15775, t15779, t15780, t15782, t15787, t15791, t15794, t15811, t15814)
}
