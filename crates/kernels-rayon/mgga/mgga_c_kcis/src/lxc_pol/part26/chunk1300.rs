//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1300/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1300(t1014: f64, t29396: f64, t1394: f64, t5644: f64, t98470: f64, t1598: f64, t23036: f64, t251: f64, t1464: f64, t20906: f64, t27387: f64, t101875: f64, t101985: f64, t102061: f64, t23097: f64, t27583: f64, t28708: f64, t28714: f64, t28721: f64, t28738: f64, t28853: f64, t7968: f64, t7978: f64, t7981: f64, t99213: f64) -> (f64, f64, f64, f64, f64) {
    let t102240 = t1014 * t29396;
    let t102245 = t1394 * t98470 * t5644;
    let t102250 = t23036 * t251 * t1598;
    let t102262 = t1464 * t27387 * t20906;
    let t102269 = -0.61905925925925925924e-2_f64 * t102240 - 0.13901041666666666667e-2_f64 * t7978 * t102061 - 0.23214722222222222222e-2_f64 * t102245 + 0.51015085286458333333e-3_f64 * t7968 * t101875 - 0.11584201388888888889e-3_f64 * t102250 * t7981 - 0.69505208333333333334e-3_f64 * t28714 * t28738 - 0.13901041666666666667e-2_f64 * t28714 * t28708 - 0.2782641015625e-3_f64 * t28721 * t28708 + 0.24734586805555555556e-3_f64 * t28853 * t28738 + 0.77382407407407407407e-3_f64 * t102262 - 0.46336805555555555556e-3_f64 * t27583 * t99213 * t23097 - 0.23168402777777777778e-3_f64 * t27583 * t101985;
    (t102240, t102245, t102250, t102262, t102269)
}
