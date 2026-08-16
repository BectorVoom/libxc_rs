//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1794;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta536(t22989: f64, t81591: f64, t22690: f64, t23153: f64, t23171: f64, t6561: f64, t80741: f64, t6643: f64, t23025: f64, t23030: f64, t23012: f64, t6653: f64, t22641: f64, t2588: f64, t225: f64, t814: f64, t6648: f64, t23021: f64, t6547: f64, t23155: f64, t23168: f64, t22893: f64, t23158: f64, t23164: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81592, t81595, t81597, t81598, t81600, t81602) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1794(t22989, t81591, t22690, t23153, t23171, t6561, t80741, t6643, t23025, t23030, t23012, t6653);
        let (t81612, t81613, t81615, t81617, t81623, t81630) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1795(t22641, t2588, t225, t814, t6648, t23021, t6547, t23155, t23168, t22893, t23158, t23164);
    (t81592, t81595, t81597, t81598, t81600, t81602, t81612, t81613, t81615, t81617, t81623, t81630)
}
