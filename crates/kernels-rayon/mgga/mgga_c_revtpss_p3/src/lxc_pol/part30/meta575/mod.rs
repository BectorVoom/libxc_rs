//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta575(t94385: f64, t94386: f64, t94383: f64, t25304: f64, t555: f64, t25898: f64, t25876: f64, t25931: f64, t25894: f64, t1444: f64, t543: f64, t268: f64, t4102: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t94388, t94390, t94391, t94392, t94394, t94395, t94398) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2025(t94385, t94386, t94383, t25304, t555, t25898, t25876, t25931, t25894, t1444, t543, t268, t4102);
    (t94388, t94390, t94391, t94392, t94394, t94395, t94398)
}
