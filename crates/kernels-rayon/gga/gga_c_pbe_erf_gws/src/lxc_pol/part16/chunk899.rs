//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 899/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk899(t5495: f64, t1046: f64, t1750: f64, t5513: f64, t1817: f64, t2612: f64, t1006: f64, t1740: f64, t1033: f64, t1778: f64, t7280: f64, t5042: f64, t5083: f64, t5085: f64, t5087: f64, t5360: f64, t7233: f64, t7249: f64, t7252: f64, t7255: f64, t7259: f64, t7262: f64, t7266: f64, t7269: f64, t7272: f64, t7278: f64, t7285: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7803 = 16.0_f64 / 45.0_f64 * t5495;
    let t7805 = 2.0_f64 / 15.0_f64 * t1750 * t1046;
    let t7806 = 8.0_f64 / 135.0_f64 * t5513;
    let t7808 = 8.0_f64 / 45.0_f64 * t2612 * t1817;
    let t7810 = 8.0_f64 / 45.0_f64 * t1006 * t1740;
    let t7811 = t1033 * t1778;
    let t7812 = 4.0_f64 / 135.0_f64 * t7811;
    let t7819 = 0.2518888888888888889e-2_f64 * t7280;
    let t7829 = t5360 + 0.16792592592592592593e-2_f64 * t5083 - 0.41981481481481481482e-3_f64 * t5087 + 0.12594444444444444445e-2_f64 * t5042 - 0.62972222222222222223e-3_f64 * t5085 + 0.83962962962962962964e-3_f64 * t7269 - 0.83962962962962962965e-3_f64 * t7278 + t7819 + 0.1385388888888888889e-1_f64 * t7272 + 0.20990740740740740742e-2_f64 * t7285 - 0.75566666666666666669e-2_f64 * t7249 - 0.50377777777777777779e-2_f64 * t7252 + 0.12594444444444444445e-2_f64 * t7255 + 0.11335e-1_f64 * t7259 + 0.15113333333333333334e-1_f64 * t7262 - 0.37783333333333333334e-2_f64 * t7266 - 0.37783333333333333334e-2_f64 * t7233;
    (t7803, t7805, t7806, t7808, t7810, t7812, t7829)
}
