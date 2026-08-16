//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3789/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3789(t487: f64, t69636: f64, t1812: f64, t3566: f64, t1209: f64, t1210: f64, t12666: f64, t1274: f64, t1277: f64, t1294: f64, t13182: f64, t17968: f64, t17973: f64, t17975: f64, t17988: f64, t17999: f64, t18108: f64, t18114: f64, t21082: f64, t21390: f64, t3569: f64, t3790: f64, t5220: f64, t5417: f64, t5422: f64, t5423: f64, t56294: f64, t56310: f64, t56314: f64, t56315: f64, t6580: f64, t6702: f64) -> f64 {
    let t72802 = t69636 * t487;
    let t72805 = t3566 * t1812;
    let t72808 = t1209 * t1812;
    let t72832 = -0.15805078039045227836e2_f64 * t56314 * t56315 * t18108 + 0.13170898365871023197e1_f64 * t72802 * t3569 - 0.52683593463484092788e1_f64 * t72805 * t17975 - 0.52683593463484092788e1_f64 * t72808 * t17988 + 0.13170898365871023197e1_f64 * t12666 * t6580 + 0.26341796731742046394e1_f64 * t18114 * t5423 + 0.13170898365871023197e1_f64 * t5220 * t17999 + 0.13170898365871023197e1_f64 * t1210 * t1277 * t21082 * t1294 - 0.52683593463484092788e1_f64 * t17973 * t56310 * t5422 - 0.39512695097613069591e1_f64 * t1274 * t13182 * t6702 * t3790 - 0.52683593463484092788e1_f64 * t56294 * t21390 - 0.79025390195226139182e1_f64 * t5417 * t17968;
    t72832
}
