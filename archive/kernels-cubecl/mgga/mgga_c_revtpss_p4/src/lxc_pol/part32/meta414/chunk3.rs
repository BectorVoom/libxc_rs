//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1441/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1441<F: Float>(t19380: F, t996: F, t6392: F, t999: F, t1079: F, t1097: F, t16305: F, t1652: F, t16600: F, t19342: F, t19351: F, t3052: F, t3264: F, t4747: F, t4752: F, t4758: F, t4764: F, t4773: F, t4778: F, t5016: F, t6351: F, t6393: F, t995: F) -> F {
    let t19381 = t996 * t19380;
    let t19384 = t6392 * t999;
    let t19385 = t1079 * t19384;
    let t19390 = -F::cast_from(0.13170898365871023197e1_f64) * t995 * t19342 - F::cast_from(0.13170898365871023197e1_f64) * t16305 * t1652 + F::cast_from(0.26341796731742046394e1_f64) * t16600 * t4758 - F::cast_from(0.65854491829355115987e0_f64) * t3052 * t6393 - F::cast_from(0.65854491829355115987e0_f64) * t19351 * t1097 + F::cast_from(0.13170898365871023197e1_f64) * t3052 * t6351 + F::cast_from(0.13170898365871023197e1_f64) * t4778 * t4764 - F::cast_from(0.13170898365871023197e1_f64) * t4747 * t4773 - F::cast_from(0.13170898365871023197e1_f64) * t4778 * t4773 - F::cast_from(0.13170898365871023197e1_f64) * t4752 * t5016 + F::cast_from(0.13170898365871023197e1_f64) * t4747 * t4764 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t19381 + F::cast_from(0.65854491829355115987e0_f64) * t995 * t19385 - F::cast_from(0.65854491829355115987e0_f64) * t3264 * t6393;
    t19390
}
