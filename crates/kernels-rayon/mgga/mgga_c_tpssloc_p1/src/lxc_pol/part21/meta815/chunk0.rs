//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2871/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2871(t10817: f64, t17510: f64, t17513: f64, t42143: f64, t17517: f64, t10771: f64, t10811: f64, t10828: f64, t14271: f64, t14328: f64, t14337: f64, t14439: f64, t14443: f64, t14463: f64, t1569: f64, t2861: f64, t2862: f64, t2880: f64, t2886: f64, t2906: f64, t2930: f64, t49285: f64, t5743: f64, t5759: f64, t5762: f64, t5775: f64, t5791: f64, t60006: f64, t60008: f64, t60010: f64, t60016: f64, t60021: f64, t60023: f64) -> (f64, f64, f64, f64) {
    let t60025 = 8.0_f64 * t10817 * t17510;
    let t60027 = 0.1929837539843104208e3_f64 * t42143 * t17513;
    let t60029 = 4.0_f64 * t10817 * t17517;
    let t60030 = 0.64327917994770140268e2_f64 * t14271 * t14439 + 0.4138081033541872024e4_f64 * t49285 * t14443 + 6.0_f64 * t2886 * t5743 * t2880 + 0.11579025239058625248e4_f64 * t10811 * t5762 * t2862 - 4.0_f64 * t2861 * t1569 * t14328 + 0.70178683471615754484e1_f64 * t14337 * t14463 + 6.0_f64 * t2886 * t5759 * t2862 - 0.14035736694323150897e2_f64 * t10828 * t5775 * t2906 + t60006 - t60008 + t60010 - 24.0_f64 * t10771 * t5743 * t2862 + t60016 + 0.35089341735807877242e1_f64 * t2930 * t5791 * t2906 - t60021 - t60023 + t60025 + t60027 + t60029;
    (t60025, t60027, t60029, t60030)
}
