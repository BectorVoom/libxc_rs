//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2229/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2229(t1035: f64, t27543: f64, t1043: f64, t1089: f64, t1096: f64, t16249: f64, t1696: f64, t25461: f64, t25464: f64, t25605: f64, t25611: f64, t25692: f64, t27415: f64, t27579: f64, t27647: f64, t27651: f64, t27664: f64, t27680: f64, t27688: f64, t3270: f64, t3325: f64, t4758: f64, t4764: f64, t4975: f64, t7102: f64, t7144: f64, t7151: f64, t7160: f64, t7167: f64, t7817: f64, t7818: f64, t93509: f64, t93901: f64, t93904: f64, t93959: f64, t94023: f64, t99877: f64) -> f64 {
    let t100431 = t1035 * t27543;
    let t100471 = -0.17347256376410398924e1_f64 * t27415 * t27680 - 0.8673628188205199462e0_f64 * t7167 * t100431 * t1043 * t1089 - 0.13170898365871023197e1_f64 * t93509 * t1696 + 0.13170898365871023197e1_f64 * t25692 * t4764 - 0.65854491829355115987e0_f64 * t93901 * t1696 - 0.52041769129231196772e1_f64 * t7144 * t25464 * t7817 * t3270 - 0.34694512752820797848e1_f64 * t7151 * t7160 * t27579 * t1096 - 0.13170898365871023197e1_f64 * t7102 * t16249 + 0.17347256376410398924e1_f64 * t93904 * t27647 + 0.17347256376410398924e1_f64 * t25611 * t27579 * t1043 * t1089 - 0.8673628188205199462e0_f64 * t93959 * t7818 + 0.17347256376410398924e1_f64 * t25605 * t99877 * t27664 + 0.26341796731742046394e1_f64 * t94023 * t4758 + 0.17347256376410398924e1_f64 * t25461 * t27688 + 0.8673628188205199462e0_f64 * t25605 * t27651 * t4975 * t3325;
    t100471
}
