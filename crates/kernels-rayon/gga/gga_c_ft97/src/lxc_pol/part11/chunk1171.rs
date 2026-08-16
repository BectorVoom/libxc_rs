//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1171/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1171(t10195: f64, t10198: f64, t10199: f64, t10823: f64, t10829: f64, t10931: f64, t10935: f64, t113: f64, t14408: f64, t1934: f64, t2904: f64, t2956: f64, t2962: f64, t2963: f64, t332: f64, t43304: f64, t43311: f64, t44642: f64, t44789: f64, t5: f64, t505: f64, t8608: f64, t889: f64, t910: f64, t911: f64) -> f64 {
    let t44795 = t889 * t910 * t8608 + 3.0_f64 * t2904 * t10935 + t889 * t10931 * t505 + 3.0_f64 / 4.0_f64 * t889 * t43304 * t332 * t113 + 3.0_f64 / 2.0_f64 * t10829 * t2963 + t43311 * t911 + 3.0_f64 / 2.0_f64 * t889 * t2962 * t113 * t2956 + 3.0_f64 * t889 * t10198 * t14408 + 3.0_f64 * t2904 * t10199 + 3.0_f64 / 2.0_f64 * t889 * t2962 * t1934 + 3.0_f64 * t2904 * t10823 + t2904 * t10195 + t5 * (t44642 + t44789) * t332 * t113 / 4.0_f64;
    t44795
}
