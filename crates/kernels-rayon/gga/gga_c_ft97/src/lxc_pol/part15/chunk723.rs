//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 723/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk723(t139: f64, t20630: f64, t527: f64, t1013: f64, t4702: f64, t8908: f64, t133: f64, t11299: f64, t15840: f64, t15855: f64, t15866: f64, t20067: f64, t20071: f64, t20074: f64, t20078: f64, t20081: f64, t20085: f64, t8914: f64) -> (f64, f64, f64, f64, f64) {
    let t20631 = t139 * t20630;
    let t20632 = t527 * t20631;
    let t20634 = t4702 * t1013;
    let t20635 = t8908 * t20634;
    let t20636 = t133 * t20635;
    let t20651 = t8914 - 0.11113000182098765433e-1_f64 * t11299 + 0.22226000364197530866e-1_f64 * t15840 - 0.33339000546296296299e-1_f64 * t15855 + 0.16669500273148148149e-1_f64 * t15866 + 0.51860667516460905352e-1_f64 * t20067 - 0.13335600218518518519e0_f64 * t20071 + 0.66678001092592592595e-1_f64 * t20074 + 0.10001700163888888889e0_f64 * t20078 - 0.10001700163888888889e0_f64 * t20081 + 0.16669500273148148149e-1_f64 * t20085;
    (t20631, t20632, t20634, t20636, t20651)
}
