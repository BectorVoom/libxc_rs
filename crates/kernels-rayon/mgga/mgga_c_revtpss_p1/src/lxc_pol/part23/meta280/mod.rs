//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1503;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1504;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta280(t10845: f64, t2487: f64, t2482: f64, t27: f64, t2719: f64, t820: f64, t843: f64, t821: f64, t235: f64, t239: f64, t231: f64, t2723: f64, t2710: f64, t826: f64, t9732: f64, t234: f64, t2735: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10846, t10850, t10858, t10866, t10867, t10868) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1503(t10845, t2487, t2482, t27, t2719, t820, t843, t821, t235);
        let (t10870, t10871) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1504(t10868, t239, t820, t231, t2723);
        let (t10885, t10886) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1505(t2710, t826, t9732, t234, t2735);
    (t10846, t10850, t10858, t10866, t10867, t10868, t10870, t10871, t10885, t10886)
}
