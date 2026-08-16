//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta815 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2871;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta815(t10817: f64, t17510: f64, t17513: f64, t42143: f64, t17517: f64, t10771: f64, t10811: f64, t10828: f64, t14271: f64, t14328: f64, t14337: f64, t14439: f64, t14443: f64, t14463: f64, t1569: f64, t2861: f64, t2862: f64, t2880: f64, t2886: f64, t2906: f64, t2930: f64, t49285: f64, t5743: f64, t5759: f64, t5762: f64, t5775: f64, t5791: f64, t60006: f64, t60008: f64, t60010: f64, t60016: f64, t60021: f64, t60023: f64, t49072: f64, t49240: f64, t912: f64, t13727: f64, t14382: f64, t14385: f64, t49489: f64, t13520: f64, t14392: f64, t14396: f64, t49274: f64, t2836: f64, t2842: f64, t5695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60025, t60027, t60029, t60030) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2871(t10817, t17510, t17513, t42143, t17517, t10771, t10811, t10828, t14271, t14328, t14337, t14439, t14443, t14463, t1569, t2861, t2862, t2880, t2886, t2906, t2930, t49285, t5743, t5759, t5762, t5775, t5791, t60006, t60008, t60010, t60016, t60021, t60023);
        let (t60033, t60035, t60037, t60039, t60041, t60044) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2872(t49072, t49240, t912, t13727, t14382, t14385, t49489, t13520, t14392, t14396, t49274, t2836, t2842, t5695);
    (t60025, t60027, t60029, t60030, t60033, t60035, t60037, t60039, t60041, t60044)
}
