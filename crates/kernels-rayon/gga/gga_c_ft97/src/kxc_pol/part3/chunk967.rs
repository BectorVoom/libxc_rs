//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 967/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk967(t18876: f64, t18943: f64, t332: f64, t113: f64, t5473: f64, t909: f64, t4381: f64, t1526: f64, t5198: f64, t9483: f64, t10915: f64, t294: f64) -> (f64, f64, f64, f64) {
    let t18944 = t18876 + t18943;
    let t18945 = t18944 * t332;
    let t18946 = t18945 * t113;
    let t18952 = t5473 * t909;
    let t18953 = t18952 * t4381;
    let t18959 = t1526 * t9483 * t5198;
    let t18961 = t10915 * t294;
    (t18946, t18953, t18959, t18961)
}
