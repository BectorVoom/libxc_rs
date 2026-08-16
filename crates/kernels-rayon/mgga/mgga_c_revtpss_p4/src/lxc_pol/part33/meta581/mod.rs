//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1992;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1993;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta581(t159: f64, t8779: f64, t218: f64, t816: f64, t10685: f64, t1946: f64, t10671: f64, t7033: f64, t25255: f64, t2689: f64, t10690: f64, t1945: f64, t9646: f64, t7030: f64, t9789: f64, t2453: f64, t2783: f64, t64: f64, t10761: f64, t9784: f64, t2482: f64, t25260: f64, t27: f64, t596: f64, t7036: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92993, t92996, t92998, t93000, t93001, t93007) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1992(t159, t8779, t218, t816, t10685, t1946, t10671, t7033, t25255, t2689, t10690, t1945, t9646);
        let (t93008, t93013, t93015, t93016, t93021, t93025, t93034) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1993(t93007, t7030, t9789, t2453, t2783, t64, t10761, t9784, t2482, t25260, t27, t596, t7036);
    (t92993, t92996, t92998, t93000, t93001, t93008, t93013, t93015, t93016, t93021, t93025, t93034)
}
