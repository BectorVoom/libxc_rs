//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2597/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2597(t1211: f64, t20721: f64, t1294: f64, t6587: f64, t1277: f64, t1210: f64, t1215: f64, t1295: f64, t1775: f64, t18037: f64, t20697: f64, t20700: f64, t20704: f64, t20710: f64, t20714: f64, t3561: f64, t3567: f64, t3572: f64, t3732: f64, t5225: f64, t5237: f64, t5251: f64, t5417: f64, t5429: f64, t5498: f64, t6580: f64, t6745: f64) -> (f64, f64, f64) {
    let t20722 = t1211 * t20721;
    let t20727 = t6587 * t1294;
    let t20728 = t1277 * t20727;
    let t20735 = -0.65854491829355115987e0_f64 * t3732 * t6745 - 0.65854491829355115987e0_f64 * t20697 * t1215 - 0.65854491829355115987e0_f64 * t20700 * t1295 + 0.13170898365871023197e1_f64 * t3567 * t20704 - 0.65854491829355115987e0_f64 * t3561 * t6745 + 0.65854491829355115987e0_f64 * t1210 * t20710 - 0.13170898365871023197e1_f64 * t3567 * t20714 - 0.13170898365871023197e1_f64 * t5417 * t5498 - 0.13170898365871023197e1_f64 * t18037 * t1775 + 0.26341796731742046394e1_f64 * t3567 * t20722 + 0.13170898365871023197e1_f64 * t5251 * t5237 + 0.65854491829355115987e0_f64 * t1210 * t20728 + 0.13170898365871023197e1_f64 * t3572 * t6580 + 0.26341796731742046394e1_f64 * t5225 * t5429;
    (t20722, t20728, t20735)
}
