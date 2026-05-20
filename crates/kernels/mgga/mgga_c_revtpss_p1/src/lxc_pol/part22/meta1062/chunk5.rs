//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3797/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3797<F: Float>(t1276: F, t6587: F, t487: F, t70208: F, t1210: F, t1215: F, t12666: F, t1277: F, t1775: F, t17964: F, t17973: F, t18005: F, t18047: F, t18109: F, t18114: F, t21618: F, t3561: F, t3567: F, t3575: F, t3584: F, t3737: F, t3738: F, t5225: F, t5246: F, t5251: F, t5498: F, t56570: F, t6573: F, t6588: F, t6744: F) -> F {
    let t73051 = t1276 * t6587;
    let t73055 = t70208 * t487;
    let t73082 = -F::cast_from(0.26341796731742046394e1_f64) * t17973 * t73051 * t3575 - F::cast_from(0.13170898365871023197e1_f64) * t73055 * t1215 - F::cast_from(0.26341796731742046394e1_f64) * t5251 * t18047 - F::cast_from(0.65854491829355115987e0_f64) * t12666 * t6588 - F::cast_from(0.26341796731742046394e1_f64) * t18005 * t5498 + F::cast_from(0.52683593463484092788e1_f64) * t5225 * t18109 + F::cast_from(0.26341796731742046394e1_f64) * t3567 * t3737 * t6573 * t3738 - F::cast_from(0.13170898365871023197e1_f64) * t5225 * t17964 + F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1277 * t6744 * t3584 - F::cast_from(0.26341796731742046394e1_f64) * t18114 * t5246 - F::cast_from(0.26341796731742046394e1_f64) * t56570 * t1775 - F::cast_from(0.13170898365871023197e1_f64) * t3561 * t21618;
    t73082
}
