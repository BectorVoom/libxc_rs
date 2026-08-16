//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1416/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1416(t1269: f64, t1770: f64, t1214: f64, t5497: f64, t1277: f64, t1211: f64, t17345: f64, t1811: f64, t3555: f64, t1210: f64, t1215: f64, t12628: f64, t12633: f64, t12641: f64, t12658: f64, t1295: f64, t13177: f64, t17331: f64, t1775: f64, t3561: f64, t3572: f64, t3576: f64, t3732: f64, t3739: f64, t495: f64, t5231: f64, t5251: f64, t5417: f64, t5423: f64, t5429: f64, t5498: f64) -> f64 {
    let t18005 = t1770 * t1269;
    let t18018 = t5497 * t1214;
    let t18019 = t1277 * t18018;
    let t18030 = t1211 * t17345;
    let t18037 = t3555 * t1811;
    let t18040 = -0.13170898365871023197e1_f64 * t18005 * t1295 + 0.13170898365871023197e1_f64 * t3572 * t5423 + 0.26341796731742046394e1_f64 * t12641 * t5231 + 0.13170898365871023197e1_f64 * t5417 * t3739 + 0.65854491829355115987e0_f64 * t17331 * t495 + 0.26341796731742046394e1_f64 * t3732 * t5429 + 0.13170898365871023197e1_f64 * t1210 * t18019 + 0.26341796731742046394e1_f64 * t3561 * t5429 + 0.13170898365871023197e1_f64 * t5251 * t3576 + 0.26341796731742046394e1_f64 * t12633 * t5231 - 0.65854491829355115987e0_f64 * t12658 * t1775 - 0.39512695097613069591e1_f64 * t12628 * t18030 - 0.13170898365871023197e1_f64 * t3561 * t5498 - 0.13170898365871023197e1_f64 * t13177 * t1775 - 0.13170898365871023197e1_f64 * t18037 * t1215;
    t18040
}
