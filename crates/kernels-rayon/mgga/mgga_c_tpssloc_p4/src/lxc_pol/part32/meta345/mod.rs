//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1385;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1386;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1387;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta345(t13550: f64, t4386: f64, t699: f64, t4339: f64, t690: f64, t4344: f64, t1540: f64, t2394: f64, t4348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13551, t13552, t13563) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1385(t13550, t4386, t699, t4339, t690);
        let t13566 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1386(t4344, t690);
        let (t13567, t13598) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1387(t13566, t1540, t2394);
        let (t13600, t13601, t13602) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1388(t13563, t13566, t4348, t690);
    (t13551, t13552, t13563, t13566, t13567, t13598, t13600, t13601, t13602)
}
