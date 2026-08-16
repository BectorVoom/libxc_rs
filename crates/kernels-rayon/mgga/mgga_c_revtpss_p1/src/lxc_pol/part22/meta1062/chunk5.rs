//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3797/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3797(t1276: f64, t6587: f64, t487: f64, t70208: f64, t1210: f64, t1215: f64, t12666: f64, t1277: f64, t1775: f64, t17964: f64, t17973: f64, t18005: f64, t18047: f64, t18109: f64, t18114: f64, t21618: f64, t3561: f64, t3567: f64, t3575: f64, t3584: f64, t3737: f64, t3738: f64, t5225: f64, t5246: f64, t5251: f64, t5498: f64, t56570: f64, t6573: f64, t6588: f64, t6744: f64) -> f64 {
    let t73051 = t1276 * t6587;
    let t73055 = t70208 * t487;
    let t73082 = -0.26341796731742046394e1_f64 * t17973 * t73051 * t3575 - 0.13170898365871023197e1_f64 * t73055 * t1215 - 0.26341796731742046394e1_f64 * t5251 * t18047 - 0.65854491829355115987e0_f64 * t12666 * t6588 - 0.26341796731742046394e1_f64 * t18005 * t5498 + 0.52683593463484092788e1_f64 * t5225 * t18109 + 0.26341796731742046394e1_f64 * t3567 * t3737 * t6573 * t3738 - 0.13170898365871023197e1_f64 * t5225 * t17964 + 0.65854491829355115987e0_f64 * t1210 * t1277 * t6744 * t3584 - 0.26341796731742046394e1_f64 * t18114 * t5246 - 0.26341796731742046394e1_f64 * t56570 * t1775 - 0.13170898365871023197e1_f64 * t3561 * t21618;
    t73082
}
