//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1347/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1347(t21342: f64, t225: f64, t494: f64, t1294: f64, t6702: f64, t13182: f64, t1210: f64, t12628: f64, t1274: f64, t1295: f64, t1775: f64, t17973: f64, t17995: f64, t18005: f64, t18065: f64, t18097: f64, t1829: f64, t20741: f64, t20744: f64, t20748: f64, t20753: f64, t20756: f64, t20760: f64, t3572: f64, t460: f64, t5220: f64, t5225: f64, t5231: f64, t5246: f64, t5498: f64, t6588: f64) -> f64 {
    let t21344 = t21342 * t225 * t494;
    let t21347 = t6702 * t1294;
    let t21348 = t13182 * t21347;
    let t21357 = -0.13170898365871023197e1_f64 * t18097 * t1775 - 0.13170898365871023197e1_f64 * t18005 * t1829 - 0.13170898365871023197e1_f64 * t1210 * t20741 - 0.26341796731742046394e1_f64 * t17973 * t20744 - 0.39512695097613069591e1_f64 * t12628 * t20748 + 0.26341796731742046394e1_f64 * t17995 * t5231 - 0.65854491829355115987e0_f64 * t20753 * t1295 - 0.13170898365871023197e1_f64 * t20756 * t1295 + 0.13170898365871023197e1_f64 * t1274 * t20760 - 0.65854491829355115987e0_f64 * t3572 * t6588 + 0.65854491829355115987e0_f64 * t460 * t21344 - 0.39512695097613069591e1_f64 * t1274 * t21348 - 0.13170898365871023197e1_f64 * t18065 * t1829 - 0.13170898365871023197e1_f64 * t5225 * t5498 - 0.13170898365871023197e1_f64 * t5220 * t5246;
    t21357
}
