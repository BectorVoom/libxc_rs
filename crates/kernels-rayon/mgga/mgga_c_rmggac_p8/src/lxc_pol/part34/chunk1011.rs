//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1011/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1011(t75469: f64, t530: f64, t71486: f64, t72132: f64, t75455: f64, t75458: f64, t75461: f64, t77605: f64, t77606: f64, t77608: f64, t77614: f64, t77620: f64, t77621: f64, t77624: f64, t77625: f64, t77626: f64, t77630: f64) -> f64 {
    let t77631 = 0.5107751987195740728e-4_f64 * t75469;
    let t77632 = -t77605 - t77606 + t77608 + t77614 + t77620 - t71486 + t77621 - 0.2363e1_f64 * t530 * t72132 - t77624 - t77625 + t77626 - 0.70077224371605468752e-6_f64 * t75455 + 0.10511583655740820313e-5_f64 * t75458 - 0.10511583655740820313e-5_f64 * t75461 - t77630 - t77631;
    t77632
}
