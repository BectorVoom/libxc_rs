//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1986/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1986(t102235: f64, t25904: f64, t102215: f64, t25878: f64, t3999: f64, t7506: f64, t102385: f64, t94383: f64, t102394: f64, t10073: f64, t26260: f64, t27836: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102615 = 0.14456046980341999104e-1_f64 * t25904 * t102235;
    let t102617 = 0.51405703062096148812e-1_f64 * t25878 * t102215;
    let t102622 = t3999 * t7506;
    let t102629 = t94383 * t102385;
    let t102634 = 0.51405703062096148812e-1_f64 * t25878 * t102394;
    let t102636 = t10073 * t27836 * t26260;
    (t102615, t102617, t102622, t102629, t102634, t102636)
}
