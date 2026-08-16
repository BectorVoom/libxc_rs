//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1799;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta509(t25924: f64, t30278: f64, t1903: f64, t8085: f64, t7296: f64, t1904: f64, t213: f64, t25930: f64, t26238: f64, t26251: f64, t26263: f64, t26279: f64, t26294: f64, t27837: f64, t28781: f64, t28783: f64, t28796: f64, t28899: f64, t30227: f64, t30248: f64, t30252: f64, t30257: f64, t30262: f64, t30267: f64, t561: f64, t6896: f64, t7295: f64, t7511: f64, t8100: f64, t30247: f64, t545: f64, t2028: f64, t2097: f64, t6918: f64, t2027: f64, t2103: f64, t26309: f64, t26361: f64, t26363: f64, t26365: f64, t28826: f64, t28838: f64, t28846: f64, t28853: f64, t28858: f64, t28895: f64, t28897: f64, t28903: f64, t28909: f64, t30071: f64, t6919: f64, t7917: f64, t8095: f64, t8104: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30279, t30282, t30283, t30286) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1799(t25924, t30278, t1903, t8085, t7296, t1904, t213, t25930, t26238, t26251, t26263, t26279, t26294, t27837, t28781, t28783, t28796, t28899, t30227, t30248, t30252, t30257, t30262, t30267, t561, t6896, t7295, t7511, t8100);
        let (t30295, t30296, t30308, t30309, t30312) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1800(t30247, t545, t2028, t2097, t6918, t7296, t2027, t2103, t26309, t26361, t26363, t26365, t27837, t28826, t28838, t28846, t28853, t28858, t28895, t28897, t28903, t28909, t30071, t6919, t7295, t7511, t7917, t8095, t8104);
    (t30279, t30282, t30283, t30286, t30295, t30296, t30308, t30309, t30312)
}
