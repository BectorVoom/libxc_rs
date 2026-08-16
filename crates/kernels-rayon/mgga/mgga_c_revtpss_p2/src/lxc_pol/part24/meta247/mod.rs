//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta247(t2689: f64, t4372: f64, t4354: f64, t9775: f64, t10722: f64, t1565: f64, t10868: f64, t241: f64, t820: f64, t2719: f64, t844: f64, t2482: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t14846, t14850, t14866, t14894, t14923, t14931) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1010(t2689, t4372, t4354, t9775, t10722, t1565, t10868, t241, t820, t2719, t844, t2482, t814);
    (t14846, t14850, t14866, t14894, t14923, t14931)
}
