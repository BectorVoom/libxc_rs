//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta174 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta174(t761: f64, t9722: f64, t2475: f64, t723: f64, t159: f64, t2461: f64, t730: f64, t167: f64, t2478: f64, t164: f64, t2479: f64, t9689: f64, t9692: f64, t9695: f64, t9698: f64, t9702: f64, t9704: f64, t9706: f64, t9709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9724, t9729, t9730, t9731, t9733, t9734, t9738, t9739, t9740, t9751) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk792(t761, t9722, t2475, t723, t159, t2461, t730, t167, t2478, t164, t2479, t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709);
    (t9724, t9729, t9730, t9731, t9733, t9734, t9738, t9739, t9740, t9751)
}
