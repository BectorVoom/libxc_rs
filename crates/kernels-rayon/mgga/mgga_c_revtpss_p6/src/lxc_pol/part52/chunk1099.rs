//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1099/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1099(t2035: f64, t34270: f64, t2007: f64, t2052: f64, t2056: f64, t28030: f64, t34250: f64, t34253: f64, t34255: f64, t34260: f64, t34263: f64, t34265: f64, t34267: f64, t34268: f64, t6985: f64, t7883: f64, t7969: f64, t7984: f64, t8463: f64) -> f64 {
    let t34271 = t34270 * t2035;
    let t34274 = -t2007 * t7969 - t2052 * t7883 - 2.0_f64 * t2056 * t28030 - 2.0_f64 * t6985 * t7984 - t34250 - t34253 - t34255 - t34260 - t34263 - t34265 - t34267 - t34268 + t34271 - t8463;
    t34274
}
