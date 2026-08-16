//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk886;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk887;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta167(t3507: f64, t491: f64, t1932: f64, t3508: f64, t1215: f64, t1235: f64, t1246: f64, t3493: f64, t1209: f64, t3032: f64, t3499: f64, t475: f64, t3590: f64, t493: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t3565: f64, t3604: f64, t3610: f64, t470: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3611, t3612, t3613, t3617, t3621, t3623, t3624) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk886(t3507, t491, t1932, t3508, t1215, t1235, t1246, t3493, t1209, t3032, t3499);
        let (t3625, t3626, t3628, t3630) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk887(t1932, t475, t3611, t3590, t493, t1201, t1244, t1247, t1249, t3565, t3604, t3610, t3613, t3617, t3621, t3624, t470, t494);
    (t3612, t3613, t3617, t3621, t3623, t3624, t3625, t3626, t3628, t3630)
}
