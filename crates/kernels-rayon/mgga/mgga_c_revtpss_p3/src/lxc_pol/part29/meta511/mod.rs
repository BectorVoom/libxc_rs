//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1831;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta511(t218: f64, t816: f64, t92993: f64, t10685: f64, t1946: f64, t10671: f64, t7033: f64, t25255: f64, t2689: f64, t10680: f64, t1945: f64, t807: f64, t10690: f64, t9646: f64, t10674: f64, t7030: f64, t9789: f64, t2453: f64, t2783: f64, t64: f64, t10761: f64, t9784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92995, t92997, t92999, t93001, t93004) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1831(t218, t816, t92993, t10685, t1946, t10671, t7033, t25255, t2689, t10680, t1945, t807);
        let (t93007, t93010, t93012, t93015, t93016, t93020) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1832(t10690, t1945, t9646, t10674, t807, t7030, t9789, t2453, t2783, t64, t10761, t9784);
    (t92995, t92997, t92999, t93001, t93004, t93007, t93010, t93012, t93015, t93016, t93020)
}
