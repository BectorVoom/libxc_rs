//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3790/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3790<F: Float>(t13181: F, t1774: F, t1214: F, t5428: F, t12633: F, t1277: F, t17967: F, t17973: F, t17986: F, t18005: F, t18043: F, t18047: F, t18059: F, t18070: F, t18073: F, t1829: F, t20704: F, t21348: F, t21366: F, t21382: F, t21389: F, t3567: F, t3568: F, t3572: F, t3732: F, t5220: F, t5251: F, t5429: F, t56486: F, t6744: F) -> F {
    let t72843 = t13181 * t1774;
    let t72861 = t5428 * t1214;
    let t72865 = -F::cast_from(0.13170898365871023197e1_f64) * t56486 * t1829 - F::cast_from(0.26341796731742046394e1_f64) * t5220 * t18047 + F::cast_from(0.52683593463484092788e1_f64) * t18005 * t5429 + F::cast_from(0.52683593463484092788e1_f64) * t18059 * t18070 + F::cast_from(0.26341796731742046394e1_f64) * t18059 * t18073 + F::cast_from(0.79025390195226139182e1_f64) * t17986 * t72843 * t17967 + F::cast_from(0.26341796731742046394e1_f64) * t3572 * t21382 - F::cast_from(0.79025390195226139182e1_f64) * t3732 * t21348 + F::cast_from(0.26341796731742046394e1_f64) * t12633 * t20704 + F::cast_from(0.26341796731742046394e1_f64) * t3572 * t21366 + F::cast_from(0.26341796731742046394e1_f64) * t5251 * t18043 - F::cast_from(0.13170898365871023197e1_f64) * t3567 * t1277 * t6744 * t3568 + F::cast_from(0.10536718692696818558e2_f64) * t17973 * t21389 * t72861;
    t72865
}
