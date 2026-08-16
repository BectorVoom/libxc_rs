//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 946/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk946(t1131: f64, t3821: f64, t2574: f64, t265: f64, t10002: f64, t5064: f64, t242: f64, t1882: f64, t5070: f64, t5181: f64, t684: f64, t724: f64) -> (f64, f64, f64, f64, f64) {
    let t18622 = t1131 * t3821;
    let t18624 = t2574 * t265 * t18622;
    let t18627 = t10002 * t5064;
    let t18628 = t242 * t18627;
    let t18633 = t1882 * t5070;
    let t18636 = t724 * t5181 * t684;
    (t18624, t18627, t18628, t18633, t18636)
}
