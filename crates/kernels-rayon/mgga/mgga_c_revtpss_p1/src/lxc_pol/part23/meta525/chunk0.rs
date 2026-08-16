//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2045/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2045(t6688: f64, t73: f64, t5458: f64, t1287: f64, t21257: f64, t1811: f64, t3766: f64, t460: f64, t3781: f64, t21040: f64, t12702: f64, t12717: f64, t12744: f64, t1285: f64, t1288: f64, t17307: f64, t17958: f64, t21416: f64, t21427: f64, t21430: f64, t21436: f64, t21439: f64, t3666: f64, t3670: f64, t3755: f64, t3767: f64, t3782: f64, t5326: f64, t5436: f64, t5443: f64, t5446: f64, t5466: f64, t5470: f64, t5481: f64, t5487: f64, t6720: f64, t6727: f64, t6738: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21442 = t6688 * t73;
    let t21443 = t21442 * t5458;
    let t21448 = t21257 * t1287;
    let t21451 = t3766 * t1811;
    let t21452 = t460 * t21451;
    let t21455 = t3781 * t1811;
    let t21456 = t460 * t21455;
    let t21459 = t21040 * t5458;
    let t21464 = -0.65854491829355115987e0_f64 * t3782 * t21416 + 0.26341796731742046394e1_f64 * t17307 * t5443 + 0.13170898365871023197e1_f64 * t12702 * t6727 - 0.13170898365871023197e1_f64 * t5326 * t5487 - 0.65854491829355115987e0_f64 * t12744 * t6738 + 0.13170898365871023197e1_f64 * t3767 * t21427 + 0.13170898365871023197e1_f64 * t3670 * t21430 + 0.13170898365871023197e1_f64 * t5436 * t5470 + 0.13170898365871023197e1_f64 * t1285 * t21436 + 0.65854491829355115987e0_f64 * t21439 * t1288 + 0.26341796731742046394e1_f64 * t12717 * t21443 - 0.13170898365871023197e1_f64 * t3666 * t6720 - 0.13170898365871023197e1_f64 * t3755 * t21448 + 0.26341796731742046394e1_f64 * t21452 * t5466 - 0.13170898365871023197e1_f64 * t21456 * t5481 - 0.65854491829355115987e0_f64 * t3755 * t21459 - 0.13170898365871023197e1_f64 * t17958 * t5446;
    (t21442, t21443, t21448, t21451, t21452, t21455, t21456, t21459, t21464)
}
