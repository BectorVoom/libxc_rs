//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1234;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1235;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta254(t6889: f64, t6891: f64, t6888: f64, t117: f64, t534: f64, t67: f64, t6559: f64, t1987: f64, t794: f64, t1372: f64, t225: f64, t567: f64, t214: f64, t1985: f64, t1377: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6892, t6893, t6896, t6897) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1234(t6889, t6891, t6888, t117, t534, t67, t6559);
        let (t6898, t6900, t6902, t6903, t6904, t6906) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1235(t1987, t794, t6897, t1372, t225, t567, t214, t1985, t1377);
        let t6907 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1236(t1385, t6906);
    (t6892, t6893, t6896, t6897, t6898, t6900, t6902, t6903, t6904, t6906, t6907)
}
