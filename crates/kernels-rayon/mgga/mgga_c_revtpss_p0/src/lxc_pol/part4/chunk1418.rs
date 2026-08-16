//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1418/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1418(t487: f64, t5216: f64, t1211: f64, t16771: f64, t16775: f64, t1210: f64, t1215: f64, t12603: f64, t1295: f64, t18043: f64, t18047: f64, t18054: f64, t18059: f64, t18062: f64, t1813: f64, t1829: f64, t3552: f64, t3556: f64, t3567: f64, t3569: f64, t3572: f64, t3585: f64, t5220: f64, t5246: f64, t5251: f64, t5423: f64) -> f64 {
    let t18065 = t5216 * t487;
    let t18070 = t1211 * t16771;
    let t18073 = t1211 * t16775;
    let t18080 = 0.13170898365871023197e1_f64 * t1210 * t18043 - 0.13170898365871023197e1_f64 * t1210 * t18047 + 0.65854491829355115987e0_f64 * t3552 * t1813 + 0.13170898365871023197e1_f64 * t3556 * t5423 - 0.13170898365871023197e1_f64 * t18054 * t1295 - 0.65854491829355115987e0_f64 * t5220 * t3585 + 0.13170898365871023197e1_f64 * t18059 * t3569 - 0.13170898365871023197e1_f64 * t18062 * t1215 - 0.13170898365871023197e1_f64 * t18065 * t1295 - 0.13170898365871023197e1_f64 * t3572 * t5246 + 0.26341796731742046394e1_f64 * t3567 * t18070 + 0.13170898365871023197e1_f64 * t3567 * t18073 - 0.13170898365871023197e1_f64 * t12603 * t1829 - 0.65854491829355115987e0_f64 * t5251 * t3585;
    t18080
}
