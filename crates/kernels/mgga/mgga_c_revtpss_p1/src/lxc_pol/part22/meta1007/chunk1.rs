//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3444/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3444<F: Float>(t15669: F, t379: F, t11190: F, t11224: F, t16314: F, t16328: F, t16371: F, t16597: F, t1696: F, t19381: F, t19396: F, t19415: F, t19425: F, t20172: F, t3052: F, t3063: F, t3075: F, t3269: F, t4773: F, t4778: F, t4947: F, t53093: F, t6251: F, t6350: F, t995: F) -> F {
    let t64711 = t15669 * t379;
    let t64722 = F::cast_from(0.52683593463484092788e1_f64) * t16371 * t4947 - F::cast_from(0.13170898365871023197e1_f64) * t995 * t3269 * t6350 * t3075 - F::cast_from(0.26341796731742046394e1_f64) * t16597 * t4773 + F::cast_from(0.26341796731742046394e1_f64) * t4778 * t16328 - F::cast_from(0.13170898365871023197e1_f64) * t53093 * t1696 + F::cast_from(0.13170898365871023197e1_f64) * t11190 * t6251 - F::cast_from(0.13170898365871023197e1_f64) * t3063 * t19381 - F::cast_from(0.52683593463484092788e1_f64) * t64711 * t16314 - F::cast_from(0.79025390195226139182e1_f64) * t3052 * t19425 + F::cast_from(0.26341796731742046394e1_f64) * t3063 * t19396 + F::cast_from(0.26341796731742046394e1_f64) * t3052 * t20172 + F::cast_from(0.26341796731742046394e1_f64) * t11224 * t19415;
    t64722
}
