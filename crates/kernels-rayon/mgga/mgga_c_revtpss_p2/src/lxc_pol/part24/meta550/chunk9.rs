//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1635/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1635(t1544: f64, t18268: f64, t2403: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t40084: f64, t40088: f64, t40099: f64, t40103: f64, t4541: f64, t5962: f64, t77341: f64, t87650: f64, t87651: f64) -> f64 {
    let t87951 = 24.0_f64 * t1544 * t4541 * t77341 - 18.0_f64 * t18268 * t2403 * t5962 + t39799 + t39807 - t39813 - t39818 - t39823 + t40084 + t40088 + t40099 + t40103 + t87650 + t87651;
    t87951
}
