//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1305/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1305(t26686: f64, t4781: f64, t4972: f64, t101501: f64, t7690: f64, t19674: f64, t283: f64, t990: f64, t101469: f64, t26685: f64, t7703: f64, t7706: f64, t93163: f64, t93662: f64, t96302: f64, t96306: f64, t96340: f64, t96345: f64, t96358: f64) -> (f64, f64) {
    let t101524 = t26686 * t4781 * t4972;
    let t101532 = t7690 * t101501;
    let t101536 = t19674 * t283 * t990;
    let t101539 = -0.18550940104166666667e-3_f64 * t26685 * t101524 - 0.69505208333333333333e-3_f64 * t7703 * t101469 + t96302 - 0.20612155671296296296e-4_f64 * t93662 - 0.30891203703703703704e-3_f64 * t96306 + 0.14739506172839506173e-2_f64 * t93163 + 0.30918233506944444444e-4_f64 * t101532 + t96340 - 0.44218518518518518516e-2_f64 * t96345 + t96358 - 0.23168402777777777778e-3_f64 * t101536 * t7706;
    (t101524, t101539)
}
