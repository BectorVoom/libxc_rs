//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta574(t26088: f64, t531: f64, t2470: f64, t26049: f64, t7284: f64, t2453: f64, t555: f64, t25898: f64, t136: f64, t137: f64, t2022: f64, t1399: f64, t2438: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t94358, t94377, t94378, t94382, t94383, t94385, t94386) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2024(t26088, t531, t2470, t26049, t7284, t2453, t555, t25898, t136, t137, t2022, t1399, t2438);
    (t94358, t94377, t94378, t94382, t94383, t94385, t94386)
}
