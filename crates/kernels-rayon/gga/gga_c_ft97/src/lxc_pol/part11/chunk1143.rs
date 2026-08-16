//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1143/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1143(t10440: f64, t8392: f64, t10491: f64, t863: f64, t309: f64, t43912: f64, t2889: f64, t8232: f64, t2869: f64, t10765: f64, t1882: f64, t3281: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44023 = t8392 * t10440;
    let t44030 = t10491 * t863;
    let t44042 = t43912 * t309;
    let t44048 = t8232 * t2889;
    let t44050 = t8232 * t2869;
    let t44052 = t1882 * t10765;
    let t44054 = t3281 * t837;
    (t44023, t44030, t44042, t44048, t44050, t44052, t44054)
}
