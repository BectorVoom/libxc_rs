//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1071/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1071(t1261: f64, t17448: f64, t17605: f64, t17792: f64, t1782: f64, t21213: f64, t21283: f64, t21285: f64, t21287: f64, t24787: f64, t24794: f64, t24798: f64, t24804: f64, t24808: f64, t3625: f64, t5373: f64, t6640: f64, t6659: f64, t6663: f64) -> f64 {
    let t24815 = 0.42874018118069736972e-3_f64 * t21283 + 0.14481890564325777821e-1_f64 * t21285 - 0.45732285992607719436e-2_f64 * t21287 - 11.0_f64 / 108.0_f64 * t21213 * t1782 + t17792 / 54.0_f64 - 0.42874018118069736972e-3_f64 * t3625 * t24787 + 0.45732285992607719436e-2_f64 * t17605 * t6640 - 0.42874018118069736972e-3_f64 * t3625 * t24794 - 0.85748036236139473944e-3_f64 * t3625 * t24798 - 0.85748036236139473944e-3_f64 * t17448 * t6640 + 0.7145669686344956162e-3_f64 * t3625 * t24804 - 0.85748036236139473944e-3_f64 * t1261 * t24808 + t5373 * t6659 / 36.0_f64 + t5373 * t6663 / 18.0_f64;
    t24815
}
