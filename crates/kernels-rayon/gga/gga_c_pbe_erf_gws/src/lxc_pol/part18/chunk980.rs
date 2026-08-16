//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 980/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk980(t5557: f64, t7946: f64, t3399: f64, t583: f64, t10561: f64, t10564: f64, t10567: f64, t10570: f64, t10573: f64, t10576: f64, t10579: f64, t10581: f64, t10583: f64, t10585: f64, t10588: f64, t5083: f64, t5360: f64, t7269: f64, t7272: f64, t7278: f64, t7819: f64) -> (f64, f64, f64, f64) {
    let t11063 = 8.0_f64 / 135.0_f64 * t5557;
    let t11064 = 16.0_f64 / 135.0_f64 * t7946;
    let t11065 = t3399 * t583;
    let t11066 = 8.0_f64 / 45.0_f64 * t11065;
    let t11082 = t5360 + 0.83962962962962962963e-3_f64 * t5083 + 0.16792592592592592593e-2_f64 * t7269 - 0.83962962962962962967e-3_f64 * t7278 + t7819 + 0.2518888888888888889e-2_f64 * t7272 - 0.41981481481481481483e-3_f64 * t10581 + 0.20990740740740740742e-2_f64 * t10561 - 0.75566666666666666669e-2_f64 * t10564 - 0.5037777777777777778e-2_f64 * t10567 + 0.12594444444444444445e-2_f64 * t10583 + 0.11335e-1_f64 * t10570 + 0.15113333333333333334e-1_f64 * t10573 - 0.62972222222222222223e-3_f64 * t10585 + 0.12594444444444444445e-2_f64 * t10576 - 0.37783333333333333334e-2_f64 * t10579 + 0.18891666666666666667e-2_f64 * t10588;
    (t11063, t11064, t11066, t11082)
}
