//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3090/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3090<F: Float>(t1210: F, t1211: F, t1215: F, t12603: F, t12650: F, t12654: F, t12666: F, t12695: F, t1274: F, t1294: F, t1295: F, t13165: F, t13177: F, t17963: F, t17968: F, t17986: F, t17987: F, t18019: F, t18109: F, t1828: F, t21389: F, t3556: F, t3561: F, t3567: F, t3569: F, t3572: F, t3737: F, t3738: F, t45438: F, t45482: F, t5231: F, t5237: F, t5245: F, t5246: F, t5429: F, t56543: F, t56555: F, t56561: F, t56570: F, t56575: F, t56588: F) -> F {
    let t56593 = F::cast_from(0.39512695097613069591e1_f64) * t45482 * t5231 + F::cast_from(0.39512695097613069591e1_f64) * t12654 * t5429 + F::cast_from(0.39512695097613069591e1_f64) * t1274 * t3737 * t17963 * t1294 + F::cast_from(0.13170898365871023197e1_f64) * t3567 * t1211 * t56543 - F::cast_from(0.39512695097613069591e1_f64) * t1210 * t3737 * t5245 * t3738 - F::cast_from(0.39512695097613069591e1_f64) * t13177 * t5246 + F::cast_from(0.19756347548806534796e1_f64) * t12666 * t5237 + F::cast_from(0.39512695097613069591e1_f64) * t3567 * t1211 * t56555 + F::cast_from(0.39512695097613069591e1_f64) * t3572 * t18019 + F::cast_from(0.15805078039045227836e2_f64) * t45438 * t1211 * t56561 - F::cast_from(0.39512695097613069591e1_f64) * t17986 * t17987 * t12650 - F::cast_from(0.11853808529283920877e2_f64) * t3561 * t17968 - F::cast_from(0.39512695097613069591e1_f64) * t56570 * t1215 + F::cast_from(0.39512695097613069591e1_f64) * t3556 * t18019 - F::cast_from(0.39512695097613069591e1_f64) * t56575 * t1295 + F::cast_from(0.79025390195226139182e1_f64) * t3561 * t18109 + F::cast_from(0.13170898365871023197e1_f64) * t1274 * t3737 * t1828 * t13165 - F::cast_from(0.39512695097613069591e1_f64) * t17986 * t21389 * t12695 + F::cast_from(0.39512695097613069591e1_f64) * t56588 * t3569 + F::cast_from(0.79025390195226139182e1_f64) * t12603 * t5429;
    t56593
}
