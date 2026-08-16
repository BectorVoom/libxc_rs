//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1738;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1739;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta425(t22643: f64, t6890: f64, t22642: f64, t225: f64, t3879: f64, t567: f64, t214: f64, t1985: f64, t1385: f64, t6992: f64, t3887: f64, t6911: f64, t3911: f64, t6906: f64, t6889: f64, t1372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22644, t22646, t22648, t22649, t22650, t22653, t22656) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1738(t22643, t6890, t22642, t225, t3879, t567, t214, t1985, t1385, t6992, t3887, t6911);
        let (t22662, t22663, t22664, t22666) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1739(t3911, t6906, t6889, t1985, t1372, t214);
    (t22644, t22646, t22648, t22649, t22650, t22653, t22656, t22662, t22663, t22664, t22666)
}
