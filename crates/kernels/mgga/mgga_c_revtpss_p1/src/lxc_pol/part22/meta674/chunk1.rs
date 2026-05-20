//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2651/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2651<F: Float>(t12702: F, t12717: F, t12744: F, t1285: F, t1288: F, t17307: F, t17958: F, t21416: F, t21427: F, t21430: F, t21436: F, t21439: F, t21443: F, t21448: F, t21452: F, t21456: F, t21459: F, t3666: F, t3670: F, t3755: F, t3767: F, t3782: F, t5326: F, t5436: F, t5443: F, t5446: F, t5466: F, t5470: F, t5481: F, t5487: F, t6720: F, t6727: F, t6738: F) -> F {
    let t21464 = -F::cast_from(0.65854491829355115987e0_f64) * t3782 * t21416 + F::cast_from(0.26341796731742046394e1_f64) * t17307 * t5443 + F::cast_from(0.13170898365871023197e1_f64) * t12702 * t6727 - F::cast_from(0.13170898365871023197e1_f64) * t5326 * t5487 - F::cast_from(0.65854491829355115987e0_f64) * t12744 * t6738 + F::cast_from(0.13170898365871023197e1_f64) * t3767 * t21427 + F::cast_from(0.13170898365871023197e1_f64) * t3670 * t21430 + F::cast_from(0.13170898365871023197e1_f64) * t5436 * t5470 + F::cast_from(0.13170898365871023197e1_f64) * t1285 * t21436 + F::cast_from(0.65854491829355115987e0_f64) * t21439 * t1288 + F::cast_from(0.26341796731742046394e1_f64) * t12717 * t21443 - F::cast_from(0.13170898365871023197e1_f64) * t3666 * t6720 - F::cast_from(0.13170898365871023197e1_f64) * t3755 * t21448 + F::cast_from(0.26341796731742046394e1_f64) * t21452 * t5466 - F::cast_from(0.13170898365871023197e1_f64) * t21456 * t5481 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t21459 - F::cast_from(0.13170898365871023197e1_f64) * t17958 * t5446;
    t21464
}
