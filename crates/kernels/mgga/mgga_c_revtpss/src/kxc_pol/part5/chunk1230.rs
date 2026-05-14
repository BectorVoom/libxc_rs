//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1230/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1230<F: Float>(t20956: F, t21506: F, t3153: F, t6688: F, t5465: F, t12709: F, t12723: F, t12751: F, t12756: F, t1285: F, t17192: F, t17861: F, t17949: F, t17958: F, t1822: F, t21465: F, t21468: F, t21473: F, t21480: F, t21484: F, t21491: F, t21495: F, t21500: F, t3746: F, t3755: F, t5436: F, t5446: F, t5459: F, t5463: F, t5466: F, t5478: F, t5491: F, t6717: F, t6731: F) -> (F, F) {
    let t21507 = t20956 * t21506;
    let t21512 = t6688 * t3153;
    let t21513 = t21512 * t5465;
    let t21516 = 0.13170898365871023197e1 * t5463 * t21465 - 0.65854491829355115987e0 * t5478 * t21468 + 0.65854491829355115987e0 * t12756 * t21473 + 0.13170898365871023197e1 * t5436 * t5491 + 0.13170898365871023197e1 * t17861 * t1822 - 0.65854491829355115987e0 * t3755 * t21480 - 0.13170898365871023197e1 * t12751 * t21484 - 0.13170898365871023197e1 * t12709 * t6717 - 0.13170898365871023197e1 * t12723 * t6717 - 0.13170898365871023197e1 * t3755 * t21491 + 0.65854491829355115987e0 * t1285 * t21495 - 0.13170898365871023197e1 * t17192 * t5446 + 0.26341796731742046394e1 * t21500 * t5466 + 0.13170898365871023197e1 * t3746 * t6731 + 0.65854491829355115987e0 * t17949 * t21507 - 0.13170898365871023197e1 * t17958 * t5459 - 0.26341796731742046394e1 * t12751 * t21513;
    (t21512, t21516)
}
