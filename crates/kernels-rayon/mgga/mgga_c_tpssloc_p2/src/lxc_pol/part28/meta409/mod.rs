//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1575;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1576;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1577;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta409(t22705: f64, t6978: f64, t22704: f64, t154: f64, t2558: f64, t1984: f64, t2010: f64, t1998: f64, t3879: f64, t214: f64, t1985: f64, t591: f64, t6896: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22706, t22707, t22715) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1575(t22705, t6978, t22704, t154, t2558);
        let t22716 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1576(t1984, t22715);
        let (t22717, t22719, t22720, t22721, t22723) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1577(t2010, t22716, t1998, t3879, t214, t1985, t154, t591);
        let t22724 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1578(t22723, t6896);
    (t22706, t22707, t22715, t22716, t22717, t22719, t22720, t22721, t22723, t22724)
}
