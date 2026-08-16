//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 436/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk436(t1788: f64, t588: f64, t592: f64, t1831: f64, t3866: f64, t1835: f64, t225: f64) -> (f64, f64, f64, f64) {
    let t5264 = t588 * t1788;
    let t5266 = t592 * t1788;
    let t5306 = t3866 * t1831;
    let t5321 = t1835 * t225;
    (t5264, t5266, t5306, t5321)
}
