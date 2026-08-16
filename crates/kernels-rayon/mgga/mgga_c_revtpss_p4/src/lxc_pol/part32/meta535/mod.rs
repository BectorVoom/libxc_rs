//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1843;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta535(t7282: f64, t9646: f64, t93139: f64, t1955: f64, t25920: f64, t4075: f64, t2028: f64, t3999: f64, t25875: f64, t25894: f64, t25877: f64, t94382: f64, t25304: f64, t25949: f64, t1419: f64, t7063: f64, t25898: f64, t9656: f64, t281: f64, t555: f64, t93238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94696, t94701, t94705, t94763, t94768, t94771) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1843(t7282, t9646, t93139, t1955, t25920, t4075, t2028, t3999, t25875, t25894, t25877, t94382);
        let (t94776, t94802, t94823, t94849, t94886) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1844(t25304, t25949, t1419, t7063, t25898, t1955, t7282, t9656, t281, t555, t93238, t25877);
    (t94696, t94701, t94705, t94763, t94768, t94771, t94776, t94802, t94823, t94849, t94886)
}
