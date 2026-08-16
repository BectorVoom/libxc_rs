//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 727/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk727(t1204: f64, t1215: f64, t1295: f64, t2144: f64, t2149: f64, t2152: f64, t460: f64, t7602: f64, t7629: f64, t7632: f64, t7636: f64, t7639: f64, t7643: f64, t7645: f64, t7648: f64, t7651: f64, t7654: f64, t7659: f64, t7662: f64, t7666: f64) -> f64 {
    let t7669 = 0.65854491829355115987e0_f64 * t1204 * t2144 - 0.65854491829355115987e0_f64 * t7602 * t1215 + 0.65854491829355115987e0_f64 * t460 * t7629 - 0.65854491829355115987e0_f64 * t7632 * t1295 - 0.8673628188205199462e0_f64 * t7636 * t7639 + 0.8673628188205199462e0_f64 * t7643 * t7645 - 0.4336814094102599731e0_f64 * t7648 * t2152 + 0.8673628188205199462e0_f64 * t7651 * t7654 - 0.4336814094102599731e0_f64 * t7659 * t7662 - 0.4336814094102599731e0_f64 * t2149 * t7666;
    t7669
}
