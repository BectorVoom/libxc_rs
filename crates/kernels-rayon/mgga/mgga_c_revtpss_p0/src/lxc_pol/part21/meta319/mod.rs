//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1594;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta319(t10988: f64, t689: f64, t2444: f64, t887: f64, t252: f64, t2769: f64, t786: f64, t2771: f64, t676: f64, t123: f64, t2435: f64, t2448: f64, t10495: f64, t10498: f64, t10501: f64, t10503: f64, t10507: f64, t10511: f64, t10513: f64, t10978: f64, t10984: f64, t10987: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10989, t10991, t10992, t10994, t10995) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1594(t10988, t689, t2444, t887, t252, t2769, t786);
        let (t10996, t10997, t10998, t11000, t11002) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1595(t2771, t676, t123, t10995, t2435, t2448, t10495, t10498, t10501, t10503, t10507, t10511, t10513, t10978, t10984, t10987, t10989, t10992, t865, t887);
    (t10989, t10991, t10992, t10994, t10995, t10996, t10997, t10998, t11000, t11002)
}
