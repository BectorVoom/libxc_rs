//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1351/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1351(t12702: f64, t12717: f64, t12744: f64, t1285: f64, t1288: f64, t17307: f64, t17958: f64, t21416: f64, t21427: f64, t21430: f64, t21436: f64, t21439: f64, t21443: f64, t21448: f64, t21452: f64, t21456: f64, t21459: f64, t3666: f64, t3670: f64, t3755: f64, t3767: f64, t3782: f64, t5326: f64, t5436: f64, t5443: f64, t5446: f64, t5466: f64, t5470: f64, t5481: f64, t5487: f64, t6720: f64, t6727: f64, t6738: f64) -> f64 {
    let t21464 = -0.65854491829355115987e0_f64 * t3782 * t21416 + 0.26341796731742046394e1_f64 * t17307 * t5443 + 0.13170898365871023197e1_f64 * t12702 * t6727 - 0.13170898365871023197e1_f64 * t5326 * t5487 - 0.65854491829355115987e0_f64 * t12744 * t6738 + 0.13170898365871023197e1_f64 * t3767 * t21427 + 0.13170898365871023197e1_f64 * t3670 * t21430 + 0.13170898365871023197e1_f64 * t5436 * t5470 + 0.13170898365871023197e1_f64 * t1285 * t21436 + 0.65854491829355115987e0_f64 * t21439 * t1288 + 0.26341796731742046394e1_f64 * t12717 * t21443 - 0.13170898365871023197e1_f64 * t3666 * t6720 - 0.13170898365871023197e1_f64 * t3755 * t21448 + 0.26341796731742046394e1_f64 * t21452 * t5466 - 0.13170898365871023197e1_f64 * t21456 * t5481 - 0.65854491829355115987e0_f64 * t3755 * t21459 - 0.13170898365871023197e1_f64 * t17958 * t5446;
    t21464
}
