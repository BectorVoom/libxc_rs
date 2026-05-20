//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3567/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3567<F: Float>(t1679: F, t994: F, t1071: F, t6235: F, t6343: F, t989: F, t1079: F, t1097: F, t11201: F, t11220: F, t16243: F, t16603: F, t16605: F, t19396: F, t20151: F, t20152: F, t20178: F, t20219: F, t3047: F, t3058: F, t3264: F, t3268: F, t3326: F, t4772: F, t4778: F, t4946: F, t6351: F, t65071: F, t65122: F, t995: F, t996: F, t999: F) -> F {
    let t68170 = t994 * t1679;
    let t68185 = t6235 * t1071;
    let t68188 = t989 * t6343;
    let t68199 = -F::cast_from(0.13170898365871023197e1_f64) * t3264 * t20152 + F::cast_from(0.13170898365871023197e1_f64) * t995 * t1079 * t20151 * t999 - F::cast_from(0.52683593463484092788e1_f64) * t68170 * t16605 + F::cast_from(0.26341796731742046394e1_f64) * t11220 * t6351 + F::cast_from(0.26341796731742046394e1_f64) * t3047 * t19396 - F::cast_from(0.65854491829355115987e0_f64) * t20178 * t3326 + F::cast_from(0.26341796731742046394e1_f64) * t3058 * t996 * t65122 - F::cast_from(0.39512695097613069591e1_f64) * t11201 * t996 * t65071 - F::cast_from(0.13170898365871023197e1_f64) * t68185 * t1097 - F::cast_from(0.13170898365871023197e1_f64) * t68188 * t1097 + F::cast_from(0.13170898365871023197e1_f64) * t3047 * t20219 - F::cast_from(0.52683593463484092788e1_f64) * t16603 * t3268 * t4772 * t4946 + F::cast_from(0.26341796731742046394e1_f64) * t4778 * t16243;
    t68199
}
