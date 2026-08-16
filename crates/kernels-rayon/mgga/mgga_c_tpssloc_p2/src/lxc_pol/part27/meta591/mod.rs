//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2050;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta591(t23041: f64, t2686: f64, t59: f64, t9971: f64, t6613: f64, t9612: f64, t23040: f64, t2617: f64, t831: f64, t23061: f64, t6604: f64, t23099: f64, t1891: f64, t1895: f64, t213: f64, t39041: f64, t1887: f64, t206: f64, t80845: f64, t23133: f64, t2703: f64, t23083: f64, t23089: f64, t23145: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81810, t81816, t81821, t81824, t81825, t81835, t81836) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2050(t23041, t2686, t59, t9971, t6613, t9612, t23040, t2617, t831, t23061, t6604, t23099);
        let (t81850, t81853, t81857, t81859, t81865) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2051(t1891, t1895, t213, t39041, t1887, t206, t80845, t23133, t2703, t23083, t23089, t23145, t2617);
    (t81810, t81816, t81821, t81824, t81825, t81835, t81836, t81850, t81853, t81857, t81859, t81865)
}
