//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3567/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3567(t1679: f64, t994: f64, t1071: f64, t6235: f64, t6343: f64, t989: f64, t1079: f64, t1097: f64, t11201: f64, t11220: f64, t16243: f64, t16603: f64, t16605: f64, t19396: f64, t20151: f64, t20152: f64, t20178: f64, t20219: f64, t3047: f64, t3058: f64, t3264: f64, t3268: f64, t3326: f64, t4772: f64, t4778: f64, t4946: f64, t6351: f64, t65071: f64, t65122: f64, t995: f64, t996: f64, t999: f64) -> f64 {
    let t68170 = t994 * t1679;
    let t68185 = t6235 * t1071;
    let t68188 = t989 * t6343;
    let t68199 = -0.13170898365871023197e1_f64 * t3264 * t20152 + 0.13170898365871023197e1_f64 * t995 * t1079 * t20151 * t999 - 0.52683593463484092788e1_f64 * t68170 * t16605 + 0.26341796731742046394e1_f64 * t11220 * t6351 + 0.26341796731742046394e1_f64 * t3047 * t19396 - 0.65854491829355115987e0_f64 * t20178 * t3326 + 0.26341796731742046394e1_f64 * t3058 * t996 * t65122 - 0.39512695097613069591e1_f64 * t11201 * t996 * t65071 - 0.13170898365871023197e1_f64 * t68185 * t1097 - 0.13170898365871023197e1_f64 * t68188 * t1097 + 0.13170898365871023197e1_f64 * t3047 * t20219 - 0.52683593463484092788e1_f64 * t16603 * t3268 * t4772 * t4946 + 0.26341796731742046394e1_f64 * t4778 * t16243;
    t68199
}
