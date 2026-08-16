//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1314;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1315;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta276(t107: f64, t2585: f64, t2281: f64, t667: f64, t2333: f64, t626: f64, t2359: f64, t655: f64, t93: f64, t94: f64, t101: f64, t102: f64, t195: f64, t40: f64, t197: f64, t52: f64, t138: f64, t2409: f64, t125: f64, t2412: f64, t701: f64, t2414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9358, t9359, t9361, t9363, t9365, t9384, t9397) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1314(t107, t2585, t2281, t667, t2333, t626, t2359, t655, t93, t94, t101, t102);
        let (t9398, t9427, t9438, t9454, t9457) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1315(t9397, t195, t40, t197, t52, t138, t2409, t125, t2412, t701, t2414);
    (t9358, t9359, t9361, t9363, t9365, t9384, t9398, t9427, t9438, t9454, t9457)
}
