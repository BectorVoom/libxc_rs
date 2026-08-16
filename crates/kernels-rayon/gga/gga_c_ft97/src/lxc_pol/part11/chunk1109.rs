//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1109/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1109(t2961: f64, t10194: f64, t10819: f64, t10829: f64, t10832: f64, t10930: f64, t10932: f64, t113: f64, t1934: f64, t2900: f64, t2904: f64, t2957: f64, t2958: f64, t2966: f64, t332: f64, t333: f64, t39370: f64, t43088: f64, t43140: f64, t43183: f64, t43270: f64, t4381: f64, t5: f64, t505: f64, t8608: f64, t886: f64, t889: f64, t909: f64) -> f64 {
    let t43290 = t2961 * t2961;
    let t43297 = 3.0_f64 / 2.0_f64 * t5 * t2900 * t1934 + t5 * t10819 * t505 + 3.0_f64 * t2904 * t10832 + 3.0_f64 * t10829 * t2966 + t2904 * t10932 + t889 * (t43088 + t43140 + t43183 + t43270) * t332 * t113 / 4.0_f64 + t5 * t886 * t8608 + 3.0_f64 / 2.0_f64 * t889 * t2957 * t1934 + t889 * t10930 * t909 * t4381 + 3.0_f64 / 2.0_f64 * t10829 * t2958 + t5 * t333 * t39370 / 4.0_f64 + t889 * t43290 * t332 * t113 / 4.0_f64 + t889 * t10194 * t505;
    t43297
}
