//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3799/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3799(t1269: f64, t20849: f64, t1210: f64, t1211: f64, t1215: f64, t12603: f64, t12658: f64, t1274: f64, t17964: f64, t17986: f64, t20697: f64, t20753: f64, t21621: f64, t225: f64, t3576: f64, t3585: f64, t3736: f64, t3737: f64, t3738: f64, t3791: f64, t45552: f64, t460: f64, t494: f64, t5245: f64, t5417: f64, t5428: f64, t6587: f64, t6588: f64, t6702: f64, t6703: f64, t70202: f64, t72098: f64) -> f64 {
    let t73137 = t20849 * t1269;
    let t73146 = -0.65854491829355115987e0_f64 * t12658 * t6588 + 0.65854491829355115987e0_f64 * t460 * t72098 * t225 * t494 - 0.13170898365871023197e1_f64 * t1210 * t3737 * t6587 * t3738 + 0.15805078039045227836e2_f64 * t1274 * t45552 * t6702 * t3738 + 0.13170898365871023197e1_f64 * t21621 * t3576 - 0.13170898365871023197e1_f64 * t5417 * t17964 - 0.65854491829355115987e0_f64 * t1210 * t1211 * t70202 - 0.52683593463484092788e1_f64 * t17986 * t3736 * t5245 * t5428 - 0.13170898365871023197e1_f64 * t73137 * t1215 - 0.65854491829355115987e0_f64 * t20753 * t3791 + 0.26341796731742046394e1_f64 * t12603 * t6703 - 0.65854491829355115987e0_f64 * t20697 * t3585;
    t73146
}
