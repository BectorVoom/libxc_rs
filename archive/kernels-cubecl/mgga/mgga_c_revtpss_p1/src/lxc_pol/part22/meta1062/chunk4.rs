//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3796/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3796<F: Float>(t1210: F, t1211: F, t12628: F, t12633: F, t1775: F, t18030: F, t18037: F, t18043: F, t18054: F, t20722: F, t21348: F, t21366: F, t21394: F, t3556: F, t3561: F, t3576: F, t3584: F, t3737: F, t45438: F, t5220: F, t5246: F, t5429: F, t56393: F, t60106: F, t6702: F, t70120: F, t71839: F) -> F {
    let t73049 = F::cast_from(0.26341796731742046394e1_f64) * t3556 * t21366 + F::cast_from(0.52683593463484092788e1_f64) * t18054 * t5429 - F::cast_from(0.79025390195226139182e1_f64) * t56393 * t18030 - F::cast_from(0.13170898365871023197e1_f64) * t60106 * t1775 - F::cast_from(0.26341796731742046394e1_f64) * t18037 * t5246 + F::cast_from(0.52683593463484092788e1_f64) * t12633 * t20722 - F::cast_from(0.79025390195226139182e1_f64) * t3561 * t21348 + F::cast_from(0.15805078039045227836e2_f64) * t45438 * t1211 * t70120 + F::cast_from(0.26341796731742046394e1_f64) * t5220 * t18043 - F::cast_from(0.13170898365871023197e1_f64) * t1210 * t3737 * t6702 * t3584 + F::cast_from(0.26341796731742046394e1_f64) * t21394 * t3576 - F::cast_from(0.39512695097613069591e1_f64) * t12628 * t1211 * t71839;
    t73049
}
