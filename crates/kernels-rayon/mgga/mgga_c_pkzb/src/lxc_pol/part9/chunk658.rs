//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 658/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk658(t2989: f64, t790: f64, t1134: f64, t1144: f64, t2957: f64, t2965: f64, t307: f64, t311: f64, t786: f64, t800: f64, t1147: f64, t2156: f64) -> (f64, f64, f64) {
    let t2990 = t790 * t2989;
    let t2993 = 0.65854491829355115987e0_f64 * t2957 * t311 - 0.65854491829355115987e0_f64 * t1134 * t800 - 0.65854491829355115987e0_f64 * t786 * t1144 + 0.13170898365871023197e1_f64 * t307 * t2965 - 0.65854491829355115987e0_f64 * t307 * t2990;
    let t2997 = t1147 * t2156;
    (t2990, t2993, t2997)
}
