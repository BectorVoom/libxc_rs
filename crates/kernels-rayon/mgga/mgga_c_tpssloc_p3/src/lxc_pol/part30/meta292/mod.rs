//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1299;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta292(t9689: f64, t9692: f64, t9695: f64, t9698: f64, t9702: f64, t9704: f64, t9706: f64, t9709: f64, t739: f64, t746: f64, t761: f64, t177: f64, t2508: f64, t2512: f64, t9490: f64, t2517: f64, t718: f64, t2475: f64, t723: f64, t159: f64, t2461: f64, t730: f64, t167: f64, t2478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9711, t9713, t9715, t9720) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1299(t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709, t739, t746, t761, t177, t2508);
        let (t9722, t9724, t9726, t9730, t9731, t9733) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1300(t2512, t9490, t9720, t761, t2517, t718, t2475, t723, t159, t2461, t730, t167, t2478);
    (t9711, t9713, t9715, t9720, t9722, t9724, t9726, t9730, t9731, t9733)
}
