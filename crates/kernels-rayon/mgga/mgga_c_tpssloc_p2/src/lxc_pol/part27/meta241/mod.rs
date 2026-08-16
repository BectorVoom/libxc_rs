//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1156;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1157;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta241(t1894: f64, t236: f64, t776: f64, t6591: f64, t2229: f64, t61: f64, t1891: f64, t133: f64, t119: f64, t212: f64, t1895: f64, t213: f64, t225: f64, t1892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6593, t6594, t6597) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1156(t1894, t236, t776, t6591, t2229, t61);
        let (t6598, t6600, t6601, t6603, t6604) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1157(t1891, t6597, t133, t119, t212, t1895, t213, t225);
        let t6605 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1158(t1892, t6604);
    (t6593, t6594, t6597, t6598, t6600, t6601, t6603, t6604, t6605)
}
