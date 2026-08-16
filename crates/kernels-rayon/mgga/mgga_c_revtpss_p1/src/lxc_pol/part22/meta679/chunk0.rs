//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2660/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2660(t21464: f64, t21516: f64, t21568: f64, t21615: f64, t1277: f64, t20849: f64, t487: f64, t1211: f64, t21082: f64, t1210: f64, t1215: f64, t12633: f64, t12641: f64, t1271: f64, t1274: f64, t18059: f64, t1813: f64, t21333: f64, t21394: f64, t21408: f64, t3732: f64, t495: f64, t5216: f64, t5220: f64, t5231: f64, t5237: f64, t5251: f64, t5417: f64, t5423: f64, t5429: f64, t6564: f64, t6574: f64, t6703: f64) -> (f64, f64, f64, f64, f64) {
    let t21617 = t21464 + t21516 + t21568 + t21615;
    let t21618 = t1277 * t21617;
    let t21621 = t20849 * t487;
    let t21624 = t1211 * t21082;
    let t21633 = -0.13170898365871023197e1_f64 * t21394 * t1215 + 0.26341796731742046394e1_f64 * t5417 * t5429 + 0.13170898365871023197e1_f64 * t5216 * t1813 + 0.13170898365871023197e1_f64 * t3732 * t6703 + 0.13170898365871023197e1_f64 * t12633 * t6574 + 0.65854491829355115987e0_f64 * t21333 * t495 + 0.26341796731742046394e1_f64 * t1274 * t21408 + 0.13170898365871023197e1_f64 * t5220 * t5237 + 0.65854491829355115987e0_f64 * t6564 * t1271 - 0.65854491829355115987e0_f64 * t1274 * t21618 - 0.65854491829355115987e0_f64 * t21621 * t1215 - 0.65854491829355115987e0_f64 * t1210 * t21624 + 0.26341796731742046394e1_f64 * t18059 * t5231 + 0.13170898365871023197e1_f64 * t5251 * t5423 + 0.13170898365871023197e1_f64 * t12641 * t6574;
    (t21617, t21618, t21621, t21624, t21633)
}
