//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta674(t90980: f64, t90993: f64, t91000: f64, t91149: f64, t91167: f64, t91305: f64, t91312: f64, t91394: f64, t91398: f64, t91078: f64, t91081: f64, t91531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93595, t93605, t93615, t93650, t93656, t93721, t93723, t93757, t93760, t93795, t93796, t93899) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2103(t90980, t90993, t91000, t91149, t91167, t91305, t91312, t91394, t91398, t91078, t91081, t91531);
    (t93595, t93605, t93615, t93650, t93656, t93721, t93723, t93757, t93760, t93795, t93796, t93899)
}
