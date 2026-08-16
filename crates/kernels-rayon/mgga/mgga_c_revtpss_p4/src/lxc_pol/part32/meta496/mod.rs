//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1772;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta496(t1444: f64, t7296: f64, t8085: f64, t8094: f64, t25924: f64, t2103: f64, t25921: f64, t26274: f64, t26279: f64, t26280: f64, t26294: f64, t26295: f64, t26302: f64, t26309: f64, t27837: f64, t28008: f64, t5728: f64, t7295: f64, t7511: f64, t7523: f64, t7528: f64, t8095: f64, t212: f64, t1358: f64, t689: f64, t2097: f64, t543: f64, t5658: f64, t7301: f64, t786: f64, t8086: f64, t1364: f64, t5774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28806, t28814, t28815, t28822) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1772(t1444, t7296, t8085, t8094, t25924, t2103, t25921, t26274, t26279, t26280, t26294, t26295, t26302, t26309, t27837, t28008, t5728, t7295, t7511, t7523, t7528, t8095);
        let (t28824, t28825, t28826, t28829, t28830, t28837, t28838, t28840) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1773(t212, t8085, t1358, t689, t2097, t543, t5658, t7301, t786, t8086, t1364, t5774);
    (t28806, t28814, t28815, t28822, t28824, t28825, t28826, t28829, t28830, t28837, t28838, t28840)
}
