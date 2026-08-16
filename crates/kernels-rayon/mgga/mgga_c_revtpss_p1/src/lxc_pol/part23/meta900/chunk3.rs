//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2864/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2864(t1555: f64, t18586: f64, t18592: f64, t18600: f64, t18603: f64, t18609: f64, t225: f64, t229: f64, t231: f64, t23227: f64, t4409: f64, t4417: f64, t4420: f64, t6006: f64, t6010: f64, t6013: f64, t73: f64, t76943: f64, t76961: f64, t76975: f64, t76981: f64, t77001: f64, t77016: f64, t77033: f64, t77061: f64, t77118: f64, t833: f64) -> f64 {
    let t77120 = (-(t76943 + t76961 + t76975 + t76981 + t77001 + t77016 + t77033 + t77061) * t225 * t229 + 3.0_f64 * t23227 * t833 + 9.0_f64 * t18586 * t1555 - 36.0_f64 * t6006 * t73 * t4417 + 9.0_f64 * t6006 * t4420 - 36.0_f64 * t4409 * t6010 + 180.0_f64 * t18592 * t18600 - 72.0_f64 * t18592 * t18603 + 9.0_f64 * t4409 * t6013 - 36.0_f64 * t18592 * t18609 + t77118) * t231;
    t77120
}
