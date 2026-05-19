//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1347/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1347<F: Float>(t21342: F, t225: F, t494: F, t1294: F, t6702: F, t13182: F, t1210: F, t12628: F, t1274: F, t1295: F, t1775: F, t17973: F, t17995: F, t18005: F, t18065: F, t18097: F, t1829: F, t20741: F, t20744: F, t20748: F, t20753: F, t20756: F, t20760: F, t3572: F, t460: F, t5220: F, t5225: F, t5231: F, t5246: F, t5498: F, t6588: F) -> F {
    let t21344 = t21342 * t225 * t494;
    let t21347 = t6702 * t1294;
    let t21348 = t13182 * t21347;
    let t21357 = -F::cast_from(0.13170898365871023197e1_f64) * t18097 * t1775 - F::cast_from(0.13170898365871023197e1_f64) * t18005 * t1829 - F::cast_from(0.13170898365871023197e1_f64) * t1210 * t20741 - F::cast_from(0.26341796731742046394e1_f64) * t17973 * t20744 - F::cast_from(0.39512695097613069591e1_f64) * t12628 * t20748 + F::cast_from(0.26341796731742046394e1_f64) * t17995 * t5231 - F::cast_from(0.65854491829355115987e0_f64) * t20753 * t1295 - F::cast_from(0.13170898365871023197e1_f64) * t20756 * t1295 + F::cast_from(0.13170898365871023197e1_f64) * t1274 * t20760 - F::cast_from(0.65854491829355115987e0_f64) * t3572 * t6588 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t21344 - F::cast_from(0.39512695097613069591e1_f64) * t1274 * t21348 - F::cast_from(0.13170898365871023197e1_f64) * t18065 * t1829 - F::cast_from(0.13170898365871023197e1_f64) * t5225 * t5498 - F::cast_from(0.13170898365871023197e1_f64) * t5220 * t5246;
    t21357
}
