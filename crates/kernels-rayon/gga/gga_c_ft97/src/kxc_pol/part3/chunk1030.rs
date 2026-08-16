//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1030/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1030(t5478: f64, t909: f64, t4381: f64, t5: f64, t5429: f64, t505: f64, t5474: f64, t1275: f64, t14571: f64, t16579: f64, t2904: f64, t333: f64, t4318: f64, t4322: f64, t4377: f64, t4382: f64, t4385: f64, t4635: f64, t5430: f64, t5475: f64, t5480: f64, t886: f64, t889: f64, t911: f64, t992: f64) -> f64 {
    let t19905 = t5478 * t909;
    let t19906 = t19905 * t4381;
    let t19920 = t5 * t5429;
    let t19927 = t5474 * t505;
    let t19939 = t889 * t19906 / 4.0_f64 + t4322 * t4382 / 2.0_f64 + t5 * t4318 * t992 / 2.0_f64 + t5 * t333 * t16579 / 4.0_f64 + t5 * t886 * t4635 / 4.0_f64 + t19920 * t911 / 4.0_f64 + t14571 * t1275 / 2.0_f64 + t2904 * t5475 / 4.0_f64 + t889 * t19927 / 4.0_f64 + t2904 * t5480 / 4.0_f64 + t4322 * t4377 / 2.0_f64 + t4322 * t4385 / 2.0_f64 + t5 * t5430 * t505 / 4.0_f64;
    t19939
}
