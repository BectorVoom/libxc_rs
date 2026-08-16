//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 807/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk807(t10869: f64, t10929: f64, t332: f64, t113: f64, t2962: f64, t505: f64, t10195: f64, t10199: f64, t10819: f64, t10823: f64, t10829: f64, t10832: f64, t1934: f64, t2900: f64, t2904: f64, t2958: f64, t2963: f64, t2966: f64, t333: f64, t5: f64, t8608: f64, t886: f64, t889: f64, t911: f64) -> (f64, f64, f64, f64, f64) {
    let t10930 = t10869 + t10929;
    let t10931 = t10930 * t332;
    let t10932 = t10931 * t113;
    let t10935 = t2962 * t505;
    let t10943 = 3.0_f64 / 4.0_f64 * t2904 * t2958 + 3.0_f64 / 2.0_f64 * t2904 * t2966 + t889 * t10195 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t889 * t10199 + t5 * t333 * t8608 / 4.0_f64 + t5 * t10819 * t113 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t889 * t10823 + 3.0_f64 / 4.0_f64 * t5 * t886 * t1934 + 3.0_f64 / 4.0_f64 * t10829 * t911 + 3.0_f64 / 4.0_f64 * t889 * t10832 + t889 * t10932 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t889 * t10935 + 3.0_f64 / 4.0_f64 * t2904 * t2963 + 3.0_f64 / 4.0_f64 * t5 * t2900 * t505;
    (t10930, t10931, t10932, t10935, t10943)
}
