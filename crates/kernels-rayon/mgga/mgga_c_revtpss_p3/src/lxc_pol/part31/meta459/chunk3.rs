//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1677/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1677(t1287: f64, t1794: f64, t5412: f64, t5245: f64, t5486: f64, t1204: f64, t1234: f64, t12717: f64, t1281: f64, t1285: f64, t17192: f64, t17289: f64, t17846: f64, t17853: f64, t1818: f64, t20850: f64, t21579: f64, t21583: f64, t21587: f64, t21592: f64, t21596: f64, t21599: f64, t3666: f64, t3670: f64, t3746: f64, t5326: f64, t5436: f64, t5449: f64, t5452: f64, t5459: f64, t5463: f64, t5474: f64, t5481: f64, t6723: f64, t6735: f64, t6741: f64) -> f64 {
    let t21607 = t5412 * t1794 * t1287;
    let t21610 = t5486 * t5245;
    let t21615 = -0.13170898365871023197e1_f64 * t5326 * t5449 - 0.13170898365871023197e1_f64 * t17289 * t1818 + 0.13170898365871023197e1_f64 * t5436 * t5474 - 0.13170898365871023197e1_f64 * t5326 * t5452 + 0.65854491829355115987e0_f64 * t1204 * t6741 - 0.13170898365871023197e1_f64 * t21579 * t5481 + 0.39512695097613069591e1_f64 * t17846 * t21583 - 0.39512695097613069591e1_f64 * t17853 * t21587 - 0.13170898365871023197e1_f64 * t17192 * t5459 + 0.26341796731742046394e1_f64 * t3670 * t21592 + 0.26341796731742046394e1_f64 * t5463 * t21596 + 0.13170898365871023197e1_f64 * t12717 * t21599 - 0.65854491829355115987e0_f64 * t20850 * t1281 + 0.65854491829355115987e0_f64 * t3746 * t6735 + 0.13170898365871023197e1_f64 * t1285 * t21607 - 0.13170898365871023197e1_f64 * t1234 * t21610 - 0.65854491829355115987e0_f64 * t3666 * t6723;
    t21615
}
