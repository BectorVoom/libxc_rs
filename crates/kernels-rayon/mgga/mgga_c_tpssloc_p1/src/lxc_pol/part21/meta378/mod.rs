//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1830;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1831;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1832;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1833;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta378(t13822: f64, t4548: f64, t973: f64, t2970: f64, t4522: f64, t6733: f64, t884: f64, t4531: f64, t10254: f64, t3961: f64, t2988: f64, t10236: f64, t10235: f64, t10186: f64, t10233: f64, t10267: f64, t10274: f64, t13806: f64, t13813: f64, t13817: f64, t2960: f64, t2986: f64, t4523: f64, t4532: f64, t4549: f64, t10189: f64, t1597: f64, t2990: f64, t2987: f64, t4540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13823, t13825, t13828, t13830, t13831, t13832, t13835) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1830(t13822, t4548, t973, t2970, t4522, t6733, t884, t4531, t10254, t3961);
        let (t13836, t13839) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1831(t13835, t2988, t10236, t3961);
        let t13845 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1832(t10235, t13839, t10186, t10233, t10267, t10274, t13806, t13813, t13817, t13825, t13830, t13832, t13836, t2960, t2986, t4523, t4532, t4549, t973);
        let t13847 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1833(t10189, t1597);
        let (t13848, t13850, t13851) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1834(t13847, t2990, t2986, t2987, t4540);
    (t13823, t13825, t13828, t13830, t13831, t13835, t13839, t13845, t13847, t13848, t13850, t13851)
}
