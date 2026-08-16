//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3799/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3799<F: Float>(t1269: F, t20849: F, t1210: F, t1211: F, t1215: F, t12603: F, t12658: F, t1274: F, t17964: F, t17986: F, t20697: F, t20753: F, t21621: F, t225: F, t3576: F, t3585: F, t3736: F, t3737: F, t3738: F, t3791: F, t45552: F, t460: F, t494: F, t5245: F, t5417: F, t5428: F, t6587: F, t6588: F, t6702: F, t6703: F, t70202: F, t72098: F) -> F {
    let t73137 = t20849 * t1269;
    let t73146 = -F::cast_from(0.65854491829355115987e0_f64) * t12658 * t6588 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t72098 * t225 * t494 - F::cast_from(0.13170898365871023197e1_f64) * t1210 * t3737 * t6587 * t3738 + F::cast_from(0.15805078039045227836e2_f64) * t1274 * t45552 * t6702 * t3738 + F::cast_from(0.13170898365871023197e1_f64) * t21621 * t3576 - F::cast_from(0.13170898365871023197e1_f64) * t5417 * t17964 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1211 * t70202 - F::cast_from(0.52683593463484092788e1_f64) * t17986 * t3736 * t5245 * t5428 - F::cast_from(0.13170898365871023197e1_f64) * t73137 * t1215 - F::cast_from(0.65854491829355115987e0_f64) * t20753 * t3791 + F::cast_from(0.26341796731742046394e1_f64) * t12603 * t6703 - F::cast_from(0.65854491829355115987e0_f64) * t20697 * t3585;
    t73146
}
