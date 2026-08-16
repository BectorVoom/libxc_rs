//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta295(t2735: f64, t546: f64, t1353: f64, t1412: f64, t808: f64, t1369: f64, t2699: f64, t1372: f64, t3943: f64, t794: f64, t3946: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9736, t9737, t9739, t9741, t9742, t9744, t9745, t9747) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1285(t2735, t546, t1353, t1412, t808, t1369, t2699, t1372, t3943, t794, t3946, t159);
    (t9736, t9737, t9739, t9741, t9742, t9744, t9745, t9747)
}
