//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 772/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk772(t1312: f64, t1518: f64, t2055: f64, t4248: f64, t7359: f64, t7889: f64, t7969: f64, t7983: f64, t7488: f64, t7900: f64, t7499: f64, t7501: f64, t7502: f64, t7504: f64, t7904: f64, t7906: f64, t7908: f64) -> (f64, f64, f64) {
    let t8075 = 2.0_f64 * t1312 * t7983 + 2.0_f64 * t1518 * t7359 + 2.0_f64 * t2055 * t4248 + 2.0_f64 * t2055 * t7889 + t7969;
    let t8079 = t7488 * t7900;
    let t8085 = -t7499 - t7904 / 24.0_f64 - t7501 + t7502 - 0.85748036236139473944e-3_f64 * t7906 - t7504 - 0.34299214494455789578e-2_f64 * t7908;
    (t8075, t8079, t8085)
}
