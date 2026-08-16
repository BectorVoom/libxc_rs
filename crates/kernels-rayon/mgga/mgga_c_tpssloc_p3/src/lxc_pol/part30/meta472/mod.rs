//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1764;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1765;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta472(t381: f64, t883: f64, t6743: f64, t23384: f64, t6790: f64, t6733: f64, t6796: f64, t995: f64, t6802: f64, t614: f64, t6794: f64, t131: f64, t350: f64, t23602: f64, t3127: f64, t1011: f64, t3131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23634, t23635) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1764(t381, t883, t6743);
        let (t23642, t23657, t23665) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1765(t23384, t6790, t6733, t6743, t6796, t995);
        let (t23666, t23668, t23669, t23670, t23677, t23678) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1766(t23665, t6802, t614, t6794, t131, t350, t23602, t3127, t1011, t3131);
    (t23634, t23635, t23642, t23657, t23665, t23666, t23668, t23669, t23670, t23677, t23678)
}
