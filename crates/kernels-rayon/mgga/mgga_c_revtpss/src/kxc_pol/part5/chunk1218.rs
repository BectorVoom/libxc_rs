//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1218/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1218(t19380: f64, t996: f64, t6392: f64, t999: f64, t1079: f64, t1097: f64, t16305: f64, t1652: f64, t16600: f64, t19342: f64, t19351: f64, t3052: f64, t3264: f64, t4747: f64, t4752: f64, t4758: f64, t4764: f64, t4773: f64, t4778: f64, t5016: f64, t6351: f64, t6393: f64, t995: f64) -> f64 {
    let t19381 = t996 * t19380;
    let t19384 = t6392 * t999;
    let t19385 = t1079 * t19384;
    let t19390 = -0.13170898365871023197e1_f64 * t995 * t19342 - 0.13170898365871023197e1_f64 * t16305 * t1652 + 0.26341796731742046394e1_f64 * t16600 * t4758 - 0.65854491829355115987e0_f64 * t3052 * t6393 - 0.65854491829355115987e0_f64 * t19351 * t1097 + 0.13170898365871023197e1_f64 * t3052 * t6351 + 0.13170898365871023197e1_f64 * t4778 * t4764 - 0.13170898365871023197e1_f64 * t4747 * t4773 - 0.13170898365871023197e1_f64 * t4778 * t4773 - 0.13170898365871023197e1_f64 * t4752 * t5016 + 0.13170898365871023197e1_f64 * t4747 * t4764 - 0.65854491829355115987e0_f64 * t995 * t19381 + 0.65854491829355115987e0_f64 * t995 * t19385 - 0.65854491829355115987e0_f64 * t3264 * t6393;
    t19390
}
