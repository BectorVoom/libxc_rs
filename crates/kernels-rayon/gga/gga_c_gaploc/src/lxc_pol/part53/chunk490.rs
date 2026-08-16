//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 490/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk490(t123: f64, t9065: f64, t488: f64, t2300: f64, t2317: f64, t6525: f64, t122: f64, t481: f64, t880: f64) -> (f64, f64, f64, f64) {
    let t9066 = t9065 * t123;
    let t9067 = t9066 * t488;
    let t9070 = t2300 * t2317;
    let t9072 = 0.23712505529730124666e-2_f64 * t6525 * t9070;
    let t9074 = t481 * t880 * t122;
    (t9066, t9067, t9072, t9074)
}
