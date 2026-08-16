//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 713/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk713(t11023: f64, t11065: f64, t1725: f64, t10937: f64, t10941: f64, t10957: f64, t10963: f64, t10988: f64, t10991: f64, t10994: f64, t10997: f64, t11001: f64, t11005: f64, t11008: f64, t11013: f64, t11016: f64, t11019: f64) -> (f64, f64) {
    let t11066 = t11023 + t11065;
    let t11067 = t11066 * t1725;
    let t11084 = -0.66228e0_f64 * t10988 + 0.33114e0_f64 * t10991 - 0.99342e0_f64 * t10994 + 0.33114e0_f64 * t10997 + 0.11038e0_f64 * t11001 - 0.73586666666666666666e-1_f64 * t11005 - 0.16557e0_f64 * t11008 - 0.60384999999999999999e0_f64 * t10957 + 0.181155e1_f64 * t10963 - 0.5519e0_f64 * t11013 - 0.16557e0_f64 * t11016 + 0.99342e0_f64 * t11019 - 0.40256666666666666668e0_f64 * t10937 + 0.20128333333333333333e0_f64 * t10941;
    (t11067, t11084)
}
