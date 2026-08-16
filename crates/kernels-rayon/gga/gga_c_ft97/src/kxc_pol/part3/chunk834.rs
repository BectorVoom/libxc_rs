//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 834/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk834(t4805: f64, t609: f64, t2179: f64, t144: f64, t379: f64, t4839: f64, t569: f64, t4824: f64, t8392: f64, t1017: f64, t18: f64, t2222: f64) -> (f64, f64, f64, f64, f64) {
    let t16977 = t4805 * t609;
    let t16978 = t2179 * t16977;
    let t16979 = t144 * t16978;
    let t16983 = t569 * t4839 * t379;
    let t16986 = t8392 * t4824;
    let t16988 = t18 * t1017;
    let t16989 = t2222 * t16988;
    (t16978, t16979, t16983, t16986, t16989)
}
