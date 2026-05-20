//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2292/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2292<F: Float>(t1269: F, t1770: F, t1214: F, t5497: F, t1277: F, t1211: F, t17345: F, t1811: F, t3555: F, t1210: F, t1215: F, t12628: F, t12633: F, t12641: F, t12658: F, t1295: F, t13177: F, t17331: F, t1775: F, t3561: F, t3572: F, t3576: F, t3732: F, t3739: F, t495: F, t5231: F, t5251: F, t5417: F, t5423: F, t5429: F, t5498: F) -> (F, F, F, F, F) {
    let t18005 = t1770 * t1269;
    let t18018 = t5497 * t1214;
    let t18019 = t1277 * t18018;
    let t18030 = t1211 * t17345;
    let t18037 = t3555 * t1811;
    let t18040 = -F::cast_from(0.13170898365871023197e1_f64) * t18005 * t1295 + F::cast_from(0.13170898365871023197e1_f64) * t3572 * t5423 + F::cast_from(0.26341796731742046394e1_f64) * t12641 * t5231 + F::cast_from(0.13170898365871023197e1_f64) * t5417 * t3739 + F::cast_from(0.65854491829355115987e0_f64) * t17331 * t495 + F::cast_from(0.26341796731742046394e1_f64) * t3732 * t5429 + F::cast_from(0.13170898365871023197e1_f64) * t1210 * t18019 + F::cast_from(0.26341796731742046394e1_f64) * t3561 * t5429 + F::cast_from(0.13170898365871023197e1_f64) * t5251 * t3576 + F::cast_from(0.26341796731742046394e1_f64) * t12633 * t5231 - F::cast_from(0.65854491829355115987e0_f64) * t12658 * t1775 - F::cast_from(0.39512695097613069591e1_f64) * t12628 * t18030 - F::cast_from(0.13170898365871023197e1_f64) * t3561 * t5498 - F::cast_from(0.13170898365871023197e1_f64) * t13177 * t1775 - F::cast_from(0.13170898365871023197e1_f64) * t18037 * t1215;
    (t18005, t18019, t18030, t18037, t18040)
}
