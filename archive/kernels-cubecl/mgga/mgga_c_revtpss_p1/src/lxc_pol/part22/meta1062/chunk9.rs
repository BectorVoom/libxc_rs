//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3801/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3801<F: Float>(t1770: F, t5412: F, t3555: F, t6695: F, t1211: F, t1215: F, t12654: F, t1277: F, t1295: F, t17986: F, t18090: F, t18097: F, t18108: F, t20700: F, t20760: F, t21389: F, t3561: F, t3567: F, t3739: F, t3790: F, t5220: F, t5231: F, t5423: F, t56588: F, t6573: F, t6703: F, t70413: F, t70422: F) -> F {
    let t73187 = t1770 * t5412;
    let t73205 = t3555 * t6695;
    let t73210 = F::cast_from(0.13170898365871023197e1_f64) * t12654 * t6703 + F::cast_from(0.52683593463484092788e1_f64) * t56588 * t5231 - F::cast_from(0.13170898365871023197e1_f64) * t3567 * t1277 * t6573 * t3790 - F::cast_from(0.26341796731742046394e1_f64) * t73187 * t1295 - F::cast_from(0.13170898365871023197e1_f64) * t5220 * t18090 - F::cast_from(0.52683593463484092788e1_f64) * t17986 * t21389 * t18108 + F::cast_from(0.26341796731742046394e1_f64) * t18097 * t5423 + F::cast_from(0.26341796731742046394e1_f64) * t3561 * t20760 + F::cast_from(0.26341796731742046394e1_f64) * t3567 * t1211 * t70413 + F::cast_from(0.13170898365871023197e1_f64) * t3567 * t1211 * t70422 - F::cast_from(0.13170898365871023197e1_f64) * t73205 * t1215 + F::cast_from(0.13170898365871023197e1_f64) * t20700 * t3739;
    t73210
}
