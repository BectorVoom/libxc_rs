//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3789/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3789<F: Float>(t487: F, t69636: F, t1812: F, t3566: F, t1209: F, t1210: F, t12666: F, t1274: F, t1277: F, t1294: F, t13182: F, t17968: F, t17973: F, t17975: F, t17988: F, t17999: F, t18108: F, t18114: F, t21082: F, t21390: F, t3569: F, t3790: F, t5220: F, t5417: F, t5422: F, t5423: F, t56294: F, t56310: F, t56314: F, t56315: F, t6580: F, t6702: F) -> F {
    let t72802 = t69636 * t487;
    let t72805 = t3566 * t1812;
    let t72808 = t1209 * t1812;
    let t72832 = -F::cast_from(0.15805078039045227836e2_f64) * t56314 * t56315 * t18108 + F::cast_from(0.13170898365871023197e1_f64) * t72802 * t3569 - F::cast_from(0.52683593463484092788e1_f64) * t72805 * t17975 - F::cast_from(0.52683593463484092788e1_f64) * t72808 * t17988 + F::cast_from(0.13170898365871023197e1_f64) * t12666 * t6580 + F::cast_from(0.26341796731742046394e1_f64) * t18114 * t5423 + F::cast_from(0.13170898365871023197e1_f64) * t5220 * t17999 + F::cast_from(0.13170898365871023197e1_f64) * t1210 * t1277 * t21082 * t1294 - F::cast_from(0.52683593463484092788e1_f64) * t17973 * t56310 * t5422 - F::cast_from(0.39512695097613069591e1_f64) * t1274 * t13182 * t6702 * t3790 - F::cast_from(0.52683593463484092788e1_f64) * t56294 * t21390 - F::cast_from(0.79025390195226139182e1_f64) * t5417 * t17968;
    t72832
}
