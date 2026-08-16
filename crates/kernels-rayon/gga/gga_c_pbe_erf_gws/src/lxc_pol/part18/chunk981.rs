//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 981/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk981(t11082: f64, t203: f64, t184: f64, t221: f64, t10801: f64, t10804: f64, t10807: f64, t10810: f64, t10813: f64, t10816: f64, t10819: f64, t10823: f64, t10825: f64, t10827: f64, t10830: f64, t4940: f64, t4941: f64, t7374: f64, t7376: f64, t7378: f64, t7549: f64) -> (f64, f64) {
    let t11083 = t203 * t11082;
    let t11084 = t11083 * t184;
    let t11086 = 2.0_f64 / 15.0_f64 * t11084 * t221;
    let t11102 = t4940 + 0.83962962962962962963e-3_f64 * t4941 + 0.16792592592592592593e-2_f64 * t7374 - 0.83962962962962962967e-3_f64 * t7378 + t7549 - 0.2518888888888888889e-2_f64 * t7376 - 0.41981481481481481483e-3_f64 * t10823 + 0.20990740740740740742e-2_f64 * t10801 - 0.75566666666666666669e-2_f64 * t10804 + 0.5037777777777777778e-2_f64 * t10807 + 0.12594444444444444445e-2_f64 * t10825 + 0.11335e-1_f64 * t10810 - 0.15113333333333333334e-1_f64 * t10813 - 0.62972222222222222223e-3_f64 * t10827 + 0.12594444444444444445e-2_f64 * t10816 - 0.37783333333333333334e-2_f64 * t10819 + 0.18891666666666666667e-2_f64 * t10830;
    (t11086, t11102)
}
