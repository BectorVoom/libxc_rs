//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta429(t6071: f64, t72: f64, t686: f64, t2465: f64, t213: f64, t6041: f64, t6048: f64, t10995: f64, t10987: f64, t11000: f64, t11004: f64, t11013: f64, t11017: f64, t11019: f64, t11030: f64, t15018: f64, t15047: f64, t15050: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18796, t18797, t18798, t18800, t18804, t18805, t18806, t18810) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1825(t6071, t72, t686, t2465, t213, t6041, t6048, t10995, t10987, t11000, t11004, t11013, t11017, t11019, t11030, t15018, t15047, t15050, t887);
    (t18796, t18797, t18798, t18800, t18804, t18805, t18806, t18810)
}
