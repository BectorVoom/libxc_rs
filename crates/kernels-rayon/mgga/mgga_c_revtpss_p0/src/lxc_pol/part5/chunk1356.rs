//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1356/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1356(t3759: f64, t6573: f64, t1234: f64, t12756: f64, t1285: f64, t1291: f64, t12966: f64, t12987: f64, t1770: f64, t1825: f64, t21333: f64, t21518: f64, t21521: f64, t21524: f64, t21527: f64, t21535: f64, t21538: f64, t21542: f64, t21551: f64, t21554: f64, t21558: f64, t21562: f64, t3670: f64, t460: f64, t490: f64, t5216: f64, t5478: f64, t5494: f64, t6564: f64, t6714: f64) -> f64 {
    let t21565 = t3759 * t6573;
    let t21568 = 0.13170898365871023197e1_f64 * t12756 * t21518 - 0.39512695097613069591e1_f64 * t12987 * t21521 + 0.26341796731742046394e1_f64 * t3670 * t21524 + 0.65854491829355115987e0_f64 * t460 * t21527 + 0.13170898365871023197e1_f64 * t5216 * t1825 + 0.13170898365871023197e1_f64 * t12966 * t6714 + 0.65854491829355115987e0_f64 * t1285 * t21535 - 0.13170898365871023197e1_f64 * t1234 * t21538 - 0.65854491829355115987e0_f64 * t1234 * t21542 + 0.13170898365871023197e1_f64 * t1770 * t5494 + 0.65854491829355115987e0_f64 * t6564 * t1291 + 0.65854491829355115987e0_f64 * t21333 * t490 - 0.65854491829355115987e0_f64 * t1234 * t21551 - 0.65854491829355115987e0_f64 * t1234 * t21554 - 0.13170898365871023197e1_f64 * t5478 * t21558 + 0.65854491829355115987e0_f64 * t1285 * t21562 + 0.13170898365871023197e1_f64 * t3670 * t21565;
    t21568
}
