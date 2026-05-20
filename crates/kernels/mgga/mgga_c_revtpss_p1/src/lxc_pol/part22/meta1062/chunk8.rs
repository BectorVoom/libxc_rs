//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3800/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3800<F: Float>(t1210: F, t1271: F, t1277: F, t13177: F, t16750: F, t1774: F, t1775: F, t17963: F, t17999: F, t18037: F, t18065: F, t18084: F, t1828: F, t20728: F, t20748: F, t21333: F, t3556: F, t3572: F, t45449: F, t495: F, t5220: F, t5237: F, t5251: F, t5429: F, t56413: F, t6580: F, t71179: F) -> F {
    let t73177 = F::cast_from(0.65854491829355115987e0_f64) * t71179 * t495 + F::cast_from(0.13170898365871023197e1_f64) * t21333 * t1271 + F::cast_from(0.52683593463484092788e1_f64) * t18065 * t5429 + F::cast_from(0.13170898365871023197e1_f64) * t3556 * t20728 + F::cast_from(0.13170898365871023197e1_f64) * t1210 * t1277 * t16750 * t1828 + F::cast_from(0.13170898365871023197e1_f64) * t5220 * t18084 + F::cast_from(0.26341796731742046394e1_f64) * t18037 * t5237 + F::cast_from(0.13170898365871023197e1_f64) * t5251 * t17999 - F::cast_from(0.79025390195226139182e1_f64) * t45449 * t20748 + F::cast_from(0.13170898365871023197e1_f64) * t1210 * t1277 * t1774 * t17963 - F::cast_from(0.13170898365871023197e1_f64) * t56413 * t1775 + F::cast_from(0.13170898365871023197e1_f64) * t3572 * t20728 + F::cast_from(0.26341796731742046394e1_f64) * t13177 * t6580;
    t73177
}
