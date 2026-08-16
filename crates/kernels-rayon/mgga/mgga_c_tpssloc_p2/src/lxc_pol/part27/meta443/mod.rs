//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1774;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1775;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1776;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta443(t22927: f64, t6897: f64, t22666: f64, t6891: f64, t6888: f64, t225: f64, t3886: f64, t3888: f64, t6889: f64, t1985: f64, t6883: f64, t6903: f64, t22870: f64, t539: f64, t12033: f64, t1375: f64, t2016: f64, t22688: f64, t22905: f64, t22908: f64, t22910: f64, t22913: f64, t22918: f64, t22922: f64, t22924: f64, t22926: f64, t3758: f64, t3889: f64, t568: f64, t6958: f64, t6963: f64, t6993: f64, t22680: f64, t533: f64, t1390: f64, t1983: f64, t2379: f64, t25: f64, t1914: f64, t193: f64, t201: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22928, t22930, t22931, t22934, t22935, t22936, t22940) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1774(t22927, t6897, t22666, t6891, t6888, t225, t3886, t3888, t6889, t1985, t6883, t6903);
        let (t22942, t22946) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1775(t22940, t22870, t539, t12033, t1375, t2016, t22688, t22905, t22908, t22910, t22913, t22918, t22922, t22924, t22926, t22928, t22931, t22936, t3758, t3889, t568, t6958, t6963, t6993);
        let (t22947, t22948, t22949, t22950, t22951, t22959) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1776(t22680, t22946, t533, t1390, t1983, t2379, t25, t1914, t193, t201);
    (t22928, t22930, t22934, t22935, t22940, t22942, t22947, t22948, t22949, t22950, t22951, t22959)
}
