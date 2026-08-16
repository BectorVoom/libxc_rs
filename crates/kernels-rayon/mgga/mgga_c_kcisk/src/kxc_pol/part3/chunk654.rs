//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 654/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk654(t10442: f64, t1801: f64, t1800: f64, t1799: f64, t213: f64, t220: f64, t967: f64) -> (f64, f64) {
    let t10443 = t1801 * t10442;
    let t10444 = t1800 * t10443;
    let t10445 = t1799 * t10444;
    let t10447 = t220 * t213;
    let t10449 = -6.0_f64 * t967 + 6.0_f64 * t10447;
    (t10445, t10449)
}
