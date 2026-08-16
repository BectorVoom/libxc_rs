//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1462;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta477(t1065: f64, t372: f64, t6299: f64, t3115: f64, t42793: f64, t6272: f64, t19675: f64, t1025: f64, t371: f64, t6276: f64, t676: f64, t15749: f64, t4858: f64, t3205: f64, t6337: f64, t15731: f64, t4879: f64, t225: f64, t64686: f64, t366: f64, t19566: f64, t3090: f64, t1086: f64, t19462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66777, t67015, t67052, t67186, t67195) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1462(t1065, t372, t6299, t3115, t42793, t6272, t19675, t1025, t371, t6276, t676, t15749, t4858);
        let (t67206, t67473, t67501, t67502, t67528, t67551) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1463(t3205, t371, t6337, t676, t15731, t4879, t225, t64686, t366, t19566, t3090, t1086, t19462);
    (t66777, t67015, t67052, t67186, t67195, t67206, t67473, t67501, t67502, t67528, t67551)
}
