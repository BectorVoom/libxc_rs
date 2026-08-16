//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1102/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1102(t25082: f64, t34302: f64, t1936: f64, t28653: f64, t34251: f64, t7359: f64, t7741: f64, t2055: f64, t34258: f64, t93: f64, t7983: f64, t8692: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34304 = 3.0_f64 * t25082 * t34302;
    let t34308 = 2.0_f64 * t28653 * t1936;
    let t34310 = 2.0_f64 * t34251 * t1936;
    let t34312 = 2.0_f64 * t7359 * t7741;
    let t34320 = 2.0_f64 * t34258 * t2055;
    let t34321 = t93 * t7741;
    let t34323 = 2.0_f64 * t34321 * t2055;
    let t34325 = 2.0_f64 * t8692 * t7983;
    (t34304, t34308, t34310, t34312, t34320, t34321, t34323, t34325)
}
