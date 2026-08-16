//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 202/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk202(t738: f64, t740: f64, t270: f64, t301: f64, t337: f64, t359: f64, t642: f64, t645: f64, t648: f64, t650: f64, t681: f64, t703: f64, t726: f64, t731: f64, t735: f64) -> (f64, f64) {
    let t741 = t738 * t740;
    let t744 = t337 + t359 + t642 - t645 - t648 + 0.10254034973522965712e-1_f64 * t650 * t301 + 0.76905262301422242837e-2_f64 * t681 * t301 - 0.76905262301422242837e-2_f64 * t270 * t703 + 0.76905262301422242837e-2_f64 * t270 * t726 - 0.85450291446024714263e-3_f64 * t731 * t735 - 0.76905262301422242837e-2_f64 * t270 * t741;
    (t741, t744)
}
