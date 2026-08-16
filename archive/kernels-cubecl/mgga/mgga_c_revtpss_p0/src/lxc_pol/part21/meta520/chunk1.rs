//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2157/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2157<F: Float>(t1079: F, t16327: F, t342: F, t4930: F, t1071: F, t1647: F, t1695: F, t3059: F, t1651: F, t3325: F, t1076: F, t1097: F, t11195: F, t16312: F, t16314: F, t16318: F, t16322: F, t1696: F, t3052: F, t3058: F, t3067: F, t3271: F, t3326: F, t4752: F, t4778: F, t4935: F, t5016: F, t995: F) -> (F, F, F, F, F, F) {
    let t16328 = t1079 * t16327;
    let t16333 = t342 * t4930;
    let t16340 = t1647 * t1071;
    let t16343 = t1695 * t3059;
    let t16344 = t1079 * t16343;
    let t16352 = t1079 * t1651 * t3325;
    let t16355 = -F::cast_from(0.26341796731742046394e1_f64) * t16312 * t16314 + F::cast_from(0.13170898365871023197e1_f64) * t1076 * t16318 - F::cast_from(0.39512695097613069591e1_f64) * t1076 * t16322 - F::cast_from(0.65854491829355115987e0_f64) * t4752 * t3326 + F::cast_from(0.13170898365871023197e1_f64) * t995 * t16328 + F::cast_from(0.13170898365871023197e1_f64) * t4935 * t3271 - F::cast_from(0.13170898365871023197e1_f64) * t16333 * t1097 + F::cast_from(0.13170898365871023197e1_f64) * t4778 * t3067 - F::cast_from(0.65854491829355115987e0_f64) * t11195 * t1696 - F::cast_from(0.13170898365871023197e1_f64) * t16340 * t1097 - F::cast_from(0.13170898365871023197e1_f64) * t3058 * t16344 - F::cast_from(0.13170898365871023197e1_f64) * t3052 * t5016 - F::cast_from(0.65854491829355115987e0_f64) * t4935 * t3326 + F::cast_from(0.65854491829355115987e0_f64) * t995 * t16352;
    (t16328, t16333, t16340, t16344, t16352, t16355)
}
