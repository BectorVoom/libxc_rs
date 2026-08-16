//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1738/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1738(t1828: f64, t3584: f64, t1277: f64, t1210: f64, t12654: f64, t1271: f64, t1274: f64, t17964: f64, t17968: f64, t17973: f64, t17975: f64, t17979: f64, t17986: f64, t17988: f64, t17992: f64, t17995: f64, t1829: f64, t3556: f64, t3569: f64, t3572: f64, t3576: f64, t3739: f64, t460: f64, t5216: f64, t5220: f64, t5225: f64, t5237: f64, t5246: f64) -> (f64, f64) {
    let t17998 = t1828 * t3584;
    let t17999 = t1277 * t17998;
    let t18004 = 0.13170898365871023197e1_f64 * t5225 * t3739 + 0.13170898365871023197e1_f64 * t5216 * t1271 - 0.65854491829355115987e0_f64 * t1274 * t17964 - 0.39512695097613069591e1_f64 * t1274 * t17968 - 0.13170898365871023197e1_f64 * t3556 * t5246 - 0.26341796731742046394e1_f64 * t17973 * t17975 + 0.65854491829355115987e0_f64 * t460 * t17979 - 0.65854491829355115987e0_f64 * t12654 * t1829 + 0.13170898365871023197e1_f64 * t5220 * t3576 - 0.26341796731742046394e1_f64 * t17986 * t17988 + 0.13170898365871023197e1_f64 * t1274 * t17992 + 0.13170898365871023197e1_f64 * t17995 * t3569 + 0.65854491829355115987e0_f64 * t1210 * t17999 + 0.13170898365871023197e1_f64 * t3572 * t5237;
    (t17999, t18004)
}
