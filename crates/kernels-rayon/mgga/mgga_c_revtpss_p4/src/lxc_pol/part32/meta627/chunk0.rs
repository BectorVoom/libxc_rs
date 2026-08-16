//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2002/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2002(t108978: f64, t2047: f64, t108986: f64, t101230: f64, t101785: f64, t101955: f64, t101969: f64, t101972: f64, t10309: f64, t108966: f64, t108975: f64, t108983: f64, t108990: f64, t25162: f64, t26175: f64, t26182: f64, t28147: f64, t28628: f64, t34764: f64) -> f64 {
    let t110039 = t2047 * t108978;
    let t110044 = t2047 * t108986;
    let t110049 = -40.0_f64 * t10309 * t34764 * t28147 - t101955 - t101969 - t101972 + 20.0_f64 * t101785 * t28147 + 20.0_f64 / 3.0_f64 * t101230 * t28628 + 20.0_f64 / 3.0_f64 * t108966 * t26182 + 20.0_f64 * t26175 * t108975 + 20.0_f64 / 3.0_f64 * t25162 * t110039 + 10.0_f64 * t26175 * t108983 + 10.0_f64 / 3.0_f64 * t25162 * t110044 + 10.0_f64 / 3.0_f64 * t108990 * t26182;
    t110049
}
