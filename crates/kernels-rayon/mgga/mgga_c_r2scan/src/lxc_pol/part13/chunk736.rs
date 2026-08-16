//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 736/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk736(t5916: f64, t722: f64, t1762: f64, t1732: f64, t1771: f64, t230: f64, t4889: f64, t5836: f64, t61: f64, t1376: f64, t725: f64, t41: f64) -> (f64, f64, f64, f64, f64) {
    let t5917 = t5916 * t722;
    let t5919 = 0.64212977516902094772e0_f64 * t1762 * t5917;
    let t5920 = t1771 * t1732;
    let t5923 = 120.0_f64 * t4889 * t230;
    let t5925 = 0.3903689268108626343e0_f64 * t61 * t5836;
    let t5926 = t1376 * t725;
    let t5927 = t41 * t5926;
    (t5919, t5920, t5923, t5925, t5927)
}
