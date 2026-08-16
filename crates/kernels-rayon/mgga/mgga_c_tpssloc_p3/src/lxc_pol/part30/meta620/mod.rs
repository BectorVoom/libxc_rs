//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta620(t26129: f64, t81442: f64, t22470: f64, t4067: f64, t111: f64, t7758: f64, t112: f64, t26509: f64, t25: f64, t40772: f64, t1408: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t86589, t86591, t86647, t86656, t86716, t86721) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2019(t26129, t81442, t22470, t4067, t111, t7758, t112, t26509, t25, t40772, t1408, t2752);
    (t86589, t86591, t86647, t86656, t86716, t86721)
}
