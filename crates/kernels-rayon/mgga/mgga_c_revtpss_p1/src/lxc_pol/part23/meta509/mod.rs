//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2003;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta509(t1248: f64, t13045: f64, t20956: f64, t3720: f64, t5341: f64, t1219: f64, t6667: f64, t247: f64, t3634: f64, t6429: f64, t1261: f64, t12856: f64, t20795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20957, t20958, t20959, t20962, t20963, t20966, t20973, t20974, t20977) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2003(t1248, t13045, t20956, t3720, t5341, t1219, t6667, t247, t3634, t6429, t1261, t12856, t20795);
    (t20957, t20958, t20959, t20962, t20963, t20966, t20973, t20974, t20977)
}
