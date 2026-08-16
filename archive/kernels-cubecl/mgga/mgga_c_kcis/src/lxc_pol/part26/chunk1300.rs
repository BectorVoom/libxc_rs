//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1300/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1300<F: Float>(t1014: F, t29396: F, t1394: F, t5644: F, t98470: F, t1598: F, t23036: F, t251: F, t1464: F, t20906: F, t27387: F, t101875: F, t101985: F, t102061: F, t23097: F, t27583: F, t28708: F, t28714: F, t28721: F, t28738: F, t28853: F, t7968: F, t7978: F, t7981: F, t99213: F) -> (F, F, F, F, F) {
    let t102240 = t1014 * t29396;
    let t102245 = t1394 * t98470 * t5644;
    let t102250 = t23036 * t251 * t1598;
    let t102262 = t1464 * t27387 * t20906;
    let t102269 = -F::cast_from(0.61905925925925925924e-2_f64) * t102240 - F::cast_from(0.13901041666666666667e-2_f64) * t7978 * t102061 - F::cast_from(0.23214722222222222222e-2_f64) * t102245 + F::cast_from(0.51015085286458333333e-3_f64) * t7968 * t101875 - F::cast_from(0.11584201388888888889e-3_f64) * t102250 * t7981 - F::cast_from(0.69505208333333333334e-3_f64) * t28714 * t28738 - F::cast_from(0.13901041666666666667e-2_f64) * t28714 * t28708 - F::cast_from(0.2782641015625e-3_f64) * t28721 * t28708 + F::cast_from(0.24734586805555555556e-3_f64) * t28853 * t28738 + F::cast_from(0.77382407407407407407e-3_f64) * t102262 - F::cast_from(0.46336805555555555556e-3_f64) * t27583 * t99213 * t23097 - F::cast_from(0.23168402777777777778e-3_f64) * t27583 * t101985;
    (t102240, t102245, t102250, t102262, t102269)
}
