//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3796/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3796(t1210: f64, t1211: f64, t12628: f64, t12633: f64, t1775: f64, t18030: f64, t18037: f64, t18043: f64, t18054: f64, t20722: f64, t21348: f64, t21366: f64, t21394: f64, t3556: f64, t3561: f64, t3576: f64, t3584: f64, t3737: f64, t45438: f64, t5220: f64, t5246: f64, t5429: f64, t56393: f64, t60106: f64, t6702: f64, t70120: f64, t71839: f64) -> f64 {
    let t73049 = 0.26341796731742046394e1_f64 * t3556 * t21366 + 0.52683593463484092788e1_f64 * t18054 * t5429 - 0.79025390195226139182e1_f64 * t56393 * t18030 - 0.13170898365871023197e1_f64 * t60106 * t1775 - 0.26341796731742046394e1_f64 * t18037 * t5246 + 0.52683593463484092788e1_f64 * t12633 * t20722 - 0.79025390195226139182e1_f64 * t3561 * t21348 + 0.15805078039045227836e2_f64 * t45438 * t1211 * t70120 + 0.26341796731742046394e1_f64 * t5220 * t18043 - 0.13170898365871023197e1_f64 * t1210 * t3737 * t6702 * t3584 + 0.26341796731742046394e1_f64 * t21394 * t3576 - 0.39512695097613069591e1_f64 * t12628 * t1211 * t71839;
    t73049
}
