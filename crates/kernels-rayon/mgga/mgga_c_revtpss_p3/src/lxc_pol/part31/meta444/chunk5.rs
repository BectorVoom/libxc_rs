//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1587/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1587(t1079: f64, t20214: f64, t1096: f64, t6258: f64, t1000: f64, t1073: f64, t1076: f64, t11201: f64, t16302: f64, t16362: f64, t1652: f64, t1680: f64, t1696: f64, t20188: f64, t20191: f64, t20195: f64, t20204: f64, t20211: f64, t3047: f64, t3063: f64, t4743: f64, t4752: f64, t4935: f64, t4947: f64, t6235: f64, t6259: f64, t995: f64) -> (f64, f64, f64) {
    let t20215 = t1079 * t20214;
    let t20218 = t6258 * t1096;
    let t20219 = t1079 * t20218;
    let t20228 = -0.39512695097613069591e1_f64 * t11201 * t20188 - 0.13170898365871023197e1_f64 * t20191 * t1000 + 0.26341796731742046394e1_f64 * t1076 * t20195 + 0.26341796731742046394e1_f64 * t4935 * t4947 - 0.13170898365871023197e1_f64 * t16362 * t1696 - 0.13170898365871023197e1_f64 * t16302 * t1652 - 0.65854491829355115987e0_f64 * t20204 * t1000 - 0.65854491829355115987e0_f64 * t3047 * t6259 + 0.13170898365871023197e1_f64 * t4743 * t1680 - 0.65854491829355115987e0_f64 * t20211 * t1000 + 0.13170898365871023197e1_f64 * t995 * t20215 + 0.65854491829355115987e0_f64 * t995 * t20219 - 0.65854491829355115987e0_f64 * t3063 * t6259 + 0.65854491829355115987e0_f64 * t6235 * t1073 + 0.26341796731742046394e1_f64 * t4752 * t4947;
    (t20215, t20219, t20228)
}
