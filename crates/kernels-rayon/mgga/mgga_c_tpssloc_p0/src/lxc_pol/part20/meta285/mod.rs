//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1480;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1481;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1482;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta285(t10743: f64, t932: f64, t2904: f64, t938: f64, t10524: f64, t951: f64, t10603: f64, t10629: f64, t315: f64, t10632: f64, t2853: f64, t923: f64, t2885: f64, t919: f64, t10717: f64, t10720: f64, t10724: f64, t10729: f64, t10733: f64, t10734: f64, t10739: f64, t10740: f64, t2856: f64, t2861: f64, t2863: f64, t2881: f64, t2886: f64, t2889: f64, t2905: f64, t2907: f64, t2930: f64, t933: f64, t943: f64, t2884: f64, t307: f64, t302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10744, t10747, t10750, t10753, t10756) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1480(t10743, t932, t2904, t938, t10524, t951, t10603, t10629, t315);
        let (t10757, t10760, t10765) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1481(t10524, t10632, t2853, t923, t2885, t919);
        let t10768 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1482(t10717, t10720, t10724, t10729, t10733, t10734, t10739, t10740, t10744, t10747, t10750, t10753, t10756, t10757, t10760, t10765, t2856, t2861, t2863, t2881, t2886, t2889, t2905, t2907, t2930, t933, t943);
        let (t10770, t10771) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1483(t2884, t307, t302);
    (t10744, t10747, t10750, t10753, t10756, t10757, t10760, t10765, t10768, t10770, t10771)
}
