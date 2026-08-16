//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 644/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk644(t1801: f64, t8939: f64, t1873: f64, t1869: f64, t2441: f64, t2527: f64) -> (f64, f64, f64, f64) {
    let t8940 = t1801 * t8939;
    let t8941 = t1873 * t8940;
    let t8942 = t1869 * t8941;
    let t8946 = t2527 * t2441;
    (t8940, t8941, t8942, t8946)
}
