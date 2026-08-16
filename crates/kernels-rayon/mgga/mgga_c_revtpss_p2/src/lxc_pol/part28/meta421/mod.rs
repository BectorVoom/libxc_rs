//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1591;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1592;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta421(t12116: f64, t4891: f64, t3172: f64, t4874: f64, t3127: f64, t4802: f64, t1063: f64, t4807: f64, t11723: f64, t11728: f64, t11730: f64, t11732: f64, t11737: f64, t11745: f64, t3106: f64, t4803: f64, t4808: f64, t4896: f64, t3153: f64, t4866: f64, t4894: f64, t3117: f64, t3133: f64, t3154: f64, t4893: f64, t13396: f64, t4801: f64, t1042: f64, t11922: f64, t4911: f64, t3115: f64, t15158: f64, t4915: f64, t1469: f64, t3075: f64, t4872: f64, t1011: f64, t11753: f64, t11756: f64, t11763: f64, t11866: f64, t3241: f64, t4892: f64, t4907: f64, t4916: f64, t4920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15769, t15772, t15775, t15779) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1591(t12116, t4891, t3172, t4874, t3127, t4802, t1063, t4807, t11723, t11728, t11730, t11732, t11737, t11745, t3106, t4803, t4808, t4896);
        let (t15780, t15782, t15787, t15791, t15794) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1592(t3153, t4866, t4894, t3117, t3133, t3154, t4893, t13396, t4801, t1042, t11922, t4911);
        let (t15811, t15814) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1593(t15794, t3115, t15158, t4915, t1469, t3075, t4872, t1042, t1011, t1063, t11753, t11756, t11763, t11866, t15782, t15787, t15791, t3127, t3241, t4892, t4907, t4916, t4920);
    (t15769, t15772, t15775, t15779, t15780, t15782, t15787, t15791, t15794, t15811, t15814)
}
