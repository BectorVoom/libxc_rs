//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3801/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3801(t1770: f64, t5412: f64, t3555: f64, t6695: f64, t1211: f64, t1215: f64, t12654: f64, t1277: f64, t1295: f64, t17986: f64, t18090: f64, t18097: f64, t18108: f64, t20700: f64, t20760: f64, t21389: f64, t3561: f64, t3567: f64, t3739: f64, t3790: f64, t5220: f64, t5231: f64, t5423: f64, t56588: f64, t6573: f64, t6703: f64, t70413: f64, t70422: f64) -> f64 {
    let t73187 = t1770 * t5412;
    let t73205 = t3555 * t6695;
    let t73210 = 0.13170898365871023197e1_f64 * t12654 * t6703 + 0.52683593463484092788e1_f64 * t56588 * t5231 - 0.13170898365871023197e1_f64 * t3567 * t1277 * t6573 * t3790 - 0.26341796731742046394e1_f64 * t73187 * t1295 - 0.13170898365871023197e1_f64 * t5220 * t18090 - 0.52683593463484092788e1_f64 * t17986 * t21389 * t18108 + 0.26341796731742046394e1_f64 * t18097 * t5423 + 0.26341796731742046394e1_f64 * t3561 * t20760 + 0.26341796731742046394e1_f64 * t3567 * t1211 * t70413 + 0.13170898365871023197e1_f64 * t3567 * t1211 * t70422 - 0.13170898365871023197e1_f64 * t73205 * t1215 + 0.13170898365871023197e1_f64 * t20700 * t3739;
    t73210
}
