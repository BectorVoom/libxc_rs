//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1415/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1415<F: Float>(t1828: F, t3584: F, t1277: F, t1210: F, t12654: F, t1271: F, t1274: F, t17964: F, t17968: F, t17973: F, t17975: F, t17979: F, t17986: F, t17988: F, t17992: F, t17995: F, t1829: F, t3556: F, t3569: F, t3572: F, t3576: F, t3739: F, t460: F, t5216: F, t5220: F, t5225: F, t5237: F, t5246: F) -> F {
    let t17998 = t1828 * t3584;
    let t17999 = t1277 * t17998;
    let t18004 = F::cast_from(0.13170898365871023197e1_f64) * t5225 * t3739 + F::cast_from(0.13170898365871023197e1_f64) * t5216 * t1271 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t17964 - F::cast_from(0.39512695097613069591e1_f64) * t1274 * t17968 - F::cast_from(0.13170898365871023197e1_f64) * t3556 * t5246 - F::cast_from(0.26341796731742046394e1_f64) * t17973 * t17975 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t17979 - F::cast_from(0.65854491829355115987e0_f64) * t12654 * t1829 + F::cast_from(0.13170898365871023197e1_f64) * t5220 * t3576 - F::cast_from(0.26341796731742046394e1_f64) * t17986 * t17988 + F::cast_from(0.13170898365871023197e1_f64) * t1274 * t17992 + F::cast_from(0.13170898365871023197e1_f64) * t17995 * t3569 + F::cast_from(0.65854491829355115987e0_f64) * t1210 * t17999 + F::cast_from(0.13170898365871023197e1_f64) * t3572 * t5237;
    t18004
}
