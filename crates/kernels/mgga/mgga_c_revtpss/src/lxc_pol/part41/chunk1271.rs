//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1271/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1271<F: Float>(t21442: F, t5458: F, t1287: F, t21257: F, t1811: F, t3766: F, t460: F, t3781: F, t21040: F, t12702: F, t12717: F, t12744: F, t1285: F, t1288: F, t17307: F, t17958: F, t21416: F, t21427: F, t21430: F, t21436: F, t21439: F, t3666: F, t3670: F, t3755: F, t3767: F, t3782: F, t5326: F, t5436: F, t5443: F, t5446: F, t5466: F, t5470: F, t5481: F, t5487: F, t6720: F, t6727: F, t6738: F) -> (F,) {
    let t21443 = t21442 * t5458;
    let t21448 = t21257 * t1287;
    let t21451 = t3766 * t1811;
    let t21452 = t460 * t21451;
    let t21455 = t3781 * t1811;
    let t21456 = t460 * t21455;
    let t21459 = t21040 * t5458;
    let t21464 = -0.65854491829355115987e0 * t3782 * t21416 + 0.26341796731742046394e1 * t17307 * t5443 + 0.13170898365871023197e1 * t12702 * t6727 - 0.13170898365871023197e1 * t5326 * t5487 - 0.65854491829355115987e0 * t12744 * t6738 + 0.13170898365871023197e1 * t3767 * t21427 + 0.13170898365871023197e1 * t3670 * t21430 + 0.13170898365871023197e1 * t5436 * t5470 + 0.13170898365871023197e1 * t1285 * t21436 + 0.65854491829355115987e0 * t21439 * t1288 + 0.26341796731742046394e1 * t12717 * t21443 - 0.13170898365871023197e1 * t3666 * t6720 - 0.13170898365871023197e1 * t3755 * t21448 + 0.26341796731742046394e1 * t21452 * t5466 - 0.13170898365871023197e1 * t21456 * t5481 - 0.65854491829355115987e0 * t3755 * t21459 - 0.13170898365871023197e1 * t17958 * t5446;
    (t21464,)
}
