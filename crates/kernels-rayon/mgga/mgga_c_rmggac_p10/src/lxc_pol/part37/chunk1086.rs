//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1086/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1086(t75733: f64, t530: f64, t71582: f64, t73255: f64, t73411: f64, t77795: f64, t77796: f64, t77797: f64, t77803: f64, t77804: f64, t77807: f64, t77810: f64, t77812: f64, t77820: f64, t77823: f64, t77824: f64, t77825: f64) -> f64 {
    let t80307 = 0.29085809927086856922e-4_f64 * t75733;
    let t80308 = -t77795 + t77796 - t77797 + t71582 + t77803 - t77804 - 0.2363e1_f64 * t530 * t73255 + t77807 + t77810 + t77812 + t73411 - t77820 + t80307 + t77823 + t77824 - t77825;
    t80308
}
