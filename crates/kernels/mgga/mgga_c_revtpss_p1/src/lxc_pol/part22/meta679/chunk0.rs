//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2660/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2660<F: Float>(t21464: F, t21516: F, t21568: F, t21615: F, t1277: F, t20849: F, t487: F, t1211: F, t21082: F, t1210: F, t1215: F, t12633: F, t12641: F, t1271: F, t1274: F, t18059: F, t1813: F, t21333: F, t21394: F, t21408: F, t3732: F, t495: F, t5216: F, t5220: F, t5231: F, t5237: F, t5251: F, t5417: F, t5423: F, t5429: F, t6564: F, t6574: F, t6703: F) -> (F, F, F, F, F) {
    let t21617 = t21464 + t21516 + t21568 + t21615;
    let t21618 = t1277 * t21617;
    let t21621 = t20849 * t487;
    let t21624 = t1211 * t21082;
    let t21633 = -F::cast_from(0.13170898365871023197e1_f64) * t21394 * t1215 + F::cast_from(0.26341796731742046394e1_f64) * t5417 * t5429 + F::cast_from(0.13170898365871023197e1_f64) * t5216 * t1813 + F::cast_from(0.13170898365871023197e1_f64) * t3732 * t6703 + F::cast_from(0.13170898365871023197e1_f64) * t12633 * t6574 + F::cast_from(0.65854491829355115987e0_f64) * t21333 * t495 + F::cast_from(0.26341796731742046394e1_f64) * t1274 * t21408 + F::cast_from(0.13170898365871023197e1_f64) * t5220 * t5237 + F::cast_from(0.65854491829355115987e0_f64) * t6564 * t1271 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t21618 - F::cast_from(0.65854491829355115987e0_f64) * t21621 * t1215 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t21624 + F::cast_from(0.26341796731742046394e1_f64) * t18059 * t5231 + F::cast_from(0.13170898365871023197e1_f64) * t5251 * t5423 + F::cast_from(0.13170898365871023197e1_f64) * t12641 * t6574;
    (t21617, t21618, t21621, t21624, t21633)
}
