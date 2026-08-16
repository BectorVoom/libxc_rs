//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2229/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2229<F: Float>(t1035: F, t27543: F, t1043: F, t1089: F, t1096: F, t16249: F, t1696: F, t25461: F, t25464: F, t25605: F, t25611: F, t25692: F, t27415: F, t27579: F, t27647: F, t27651: F, t27664: F, t27680: F, t27688: F, t3270: F, t3325: F, t4758: F, t4764: F, t4975: F, t7102: F, t7144: F, t7151: F, t7160: F, t7167: F, t7817: F, t7818: F, t93509: F, t93901: F, t93904: F, t93959: F, t94023: F, t99877: F) -> F {
    let t100431 = t1035 * t27543;
    let t100471 = -F::cast_from(0.17347256376410398924e1_f64) * t27415 * t27680 - F::cast_from(0.8673628188205199462e0_f64) * t7167 * t100431 * t1043 * t1089 - F::cast_from(0.13170898365871023197e1_f64) * t93509 * t1696 + F::cast_from(0.13170898365871023197e1_f64) * t25692 * t4764 - F::cast_from(0.65854491829355115987e0_f64) * t93901 * t1696 - F::cast_from(0.52041769129231196772e1_f64) * t7144 * t25464 * t7817 * t3270 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t27579 * t1096 - F::cast_from(0.13170898365871023197e1_f64) * t7102 * t16249 + F::cast_from(0.17347256376410398924e1_f64) * t93904 * t27647 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t27579 * t1043 * t1089 - F::cast_from(0.8673628188205199462e0_f64) * t93959 * t7818 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t99877 * t27664 + F::cast_from(0.26341796731742046394e1_f64) * t94023 * t4758 + F::cast_from(0.17347256376410398924e1_f64) * t25461 * t27688 + F::cast_from(0.8673628188205199462e0_f64) * t25605 * t27651 * t4975 * t3325;
    t100471
}
