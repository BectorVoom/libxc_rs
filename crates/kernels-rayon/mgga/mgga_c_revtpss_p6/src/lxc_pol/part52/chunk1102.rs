//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1102/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1102(t2055: f64, t34321: f64, t7983: f64, t8692: f64, t1518: f64, t28030: f64, t32389: f64, t33602: f64, t33644: f64, t33646: f64, t34188: f64, t34308: f64, t34310: f64, t34312: f64, t34320: f64, t6985: f64, t8564: f64) -> f64 {
    let t34323 = 2.0_f64 * t34321 * t2055;
    let t34325 = 2.0_f64 * t8692 * t7983;
    let t34326 = 2.0_f64 * t1518 * t32389 + 2.0_f64 * t2055 * t28030 + 2.0_f64 * t2055 * t33602 + 2.0_f64 * t6985 * t7983 + t33644 + t33646 + t34188 + t34308 + t34310 + t34312 + t34320 + t34323 + t34325 + t8564;
    t34326
}
