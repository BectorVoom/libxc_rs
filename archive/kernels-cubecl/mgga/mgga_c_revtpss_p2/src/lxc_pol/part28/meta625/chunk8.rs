//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2231/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2231<F: Float>(t11239: F, t1678: F, t1078: F, t1982: F, t1096: F, t16287: F, t16292: F, t16322: F, t1651: F, t1652: F, t25464: F, t25466: F, t25586: F, t25591: F, t25651: F, t25674: F, t25678: F, t25692: F, t27422: F, t27545: F, t27609: F, t27679: F, t3059: F, t3325: F, t4773: F, t7102: F, t7140: F, t7144: F, t7145: F, t7151: F, t7159: F, t7160: F, t7817: F, t7821: F, t7828: F, t93881: F, t93994: F, t989: F, t999: F) -> F {
    let t100533 = t1678 * t11239;
    let t100535 = t1982 * t100533 * t1078;
    let t100560 = F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t27679 * t1096 - F::cast_from(0.26020884564615598386e1_f64) * t7159 * t25464 * t7828 * t3325 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t7145 * t27422 * t999 + F::cast_from(0.10408353825846239354e2_f64) * t93994 * t7145 * t7821 * t3059 + F::cast_from(0.26341796731742046394e1_f64) * t25651 * t16292 - F::cast_from(0.8673628188205199462e0_f64) * t100535 * t25674 + F::cast_from(0.4336814094102599731e0_f64) * t100535 * t25678 - F::cast_from(0.65854491829355115987e0_f64) * t7102 * t16287 - F::cast_from(0.65854491829355115987e0_f64) * t93881 * t1652 - F::cast_from(0.13170898365871023197e1_f64) * t25692 * t4773 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t7145 * t25586 * t1651 - F::cast_from(0.26020884564615598386e1_f64) * t27609 * t25466 + F::cast_from(0.17347256376410398924e1_f64) * t7144 * t7160 * t7817 * t3325 - F::cast_from(0.39512695097613069591e1_f64) * t7140 * t16322 + F::cast_from(0.13170898365871023197e1_f64) * t989 * t27545;
    t100560
}
