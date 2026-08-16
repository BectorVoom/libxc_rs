//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta996 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3385;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3386;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta996(t19256: f64, t41583: f64, t11385: f64, t19255: f64, t2918: f64, t2875: f64, t41499: f64, t41502: f64, t6109: f64, t4707: f64, t972: f64, t4711: f64, t52238: f64, t5019: f64, t11591: f64, t6227: f64, t6219: f64, t19077: f64, t914: f64, t936: f64, t15235: f64, t4724: f64, t981: f64, t41588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63589, t63592, t63596, t63597, t63600) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3385(t19256, t41583, t11385, t19255, t2918, t2875, t41499, t41502, t6109, t4707, t972, t4711, t52238);
        let (t63601, t63607, t63609, t63612, t63615, t63618) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3386(t5019, t11591, t6227, t6219, t19077, t914, t936, t15235, t4724, t981, t19255, t2875, t41588);
    (t63589, t63592, t63596, t63597, t63600, t63601, t63607, t63609, t63612, t63615, t63618)
}
