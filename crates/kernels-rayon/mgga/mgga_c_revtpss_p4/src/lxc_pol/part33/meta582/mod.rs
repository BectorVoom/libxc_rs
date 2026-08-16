//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta582(t2487: f64, t93034: f64, t2681: f64, t7036: f64, t820: f64, t839: f64, t25260: f64, t843: f64, t10867: f64, t64: f64, t7043: f64, t857: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t93035, t93048, t93049, t93054, t93060, t93066, t93067) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1994(t2487, t93034, t2681, t7036, t820, t839, t25260, t843, t10867, t64, t7043, t857);
    (t93035, t93048, t93049, t93054, t93060, t93066, t93067)
}
