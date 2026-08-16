//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2231/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2231(t11239: f64, t1678: f64, t1078: f64, t1982: f64, t1096: f64, t16287: f64, t16292: f64, t16322: f64, t1651: f64, t1652: f64, t25464: f64, t25466: f64, t25586: f64, t25591: f64, t25651: f64, t25674: f64, t25678: f64, t25692: f64, t27422: f64, t27545: f64, t27609: f64, t27679: f64, t3059: f64, t3325: f64, t4773: f64, t7102: f64, t7140: f64, t7144: f64, t7145: f64, t7151: f64, t7159: f64, t7160: f64, t7817: f64, t7821: f64, t7828: f64, t93881: f64, t93994: f64, t989: f64, t999: f64) -> f64 {
    let t100533 = t1678 * t11239;
    let t100535 = t1982 * t100533 * t1078;
    let t100560 = 0.34694512752820797848e1_f64 * t7144 * t7160 * t27679 * t1096 - 0.26020884564615598386e1_f64 * t7159 * t25464 * t7828 * t3325 + 0.34694512752820797848e1_f64 * t25591 * t7145 * t27422 * t999 + 0.10408353825846239354e2_f64 * t93994 * t7145 * t7821 * t3059 + 0.26341796731742046394e1_f64 * t25651 * t16292 - 0.8673628188205199462e0_f64 * t100535 * t25674 + 0.4336814094102599731e0_f64 * t100535 * t25678 - 0.65854491829355115987e0_f64 * t7102 * t16287 - 0.65854491829355115987e0_f64 * t93881 * t1652 - 0.13170898365871023197e1_f64 * t25692 * t4773 + 0.8673628188205199462e0_f64 * t7151 * t7145 * t25586 * t1651 - 0.26020884564615598386e1_f64 * t27609 * t25466 + 0.17347256376410398924e1_f64 * t7144 * t7160 * t7817 * t3325 - 0.39512695097613069591e1_f64 * t7140 * t16322 + 0.13170898365871023197e1_f64 * t989 * t27545;
    t100560
}
