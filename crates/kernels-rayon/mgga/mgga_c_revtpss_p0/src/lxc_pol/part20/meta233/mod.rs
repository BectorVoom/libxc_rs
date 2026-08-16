//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1030;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta233(t10868: f64, t239: f64, t820: f64, t231: f64, t2723: f64, t10665: f64, t827: f64, t828: f64, t10666: f64, t2648: f64, t2741: f64, t2710: f64, t826: f64, t9732: f64, t234: f64, t2735: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10870, t10871) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1030(t10868, t239, t820, t231, t2723);
        let (t10872, t10874, t10878, t10881, t10885, t10886) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1031(t10665, t10871, t827, t828, t10666, t2648, t2741, t2710, t826, t9732, t234, t2735);
    (t10870, t10871, t10872, t10874, t10878, t10881, t10885, t10886)
}
