//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 778/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk778(t1770: f64, t1775: f64, t1829: f64, t2144: f64, t2149: f64, t2152: f64, t460: f64, t7602: f64, t7632: f64, t7636: f64, t7643: f64, t7651: f64, t7659: f64, t8192: f64, t8198: f64, t8202: f64, t8205: f64, t8209: f64, t8213: f64, t8217: f64) -> f64 {
    let t8220 = 0.65854491829355115987e0_f64 * t1770 * t2144 - 0.65854491829355115987e0_f64 * t7602 * t1775 + 0.65854491829355115987e0_f64 * t460 * t8192 - 0.65854491829355115987e0_f64 * t7632 * t1829 - 0.8673628188205199462e0_f64 * t7636 * t8198 + 0.8673628188205199462e0_f64 * t7643 * t8202 - 0.4336814094102599731e0_f64 * t8205 * t2152 + 0.8673628188205199462e0_f64 * t7651 * t8209 - 0.4336814094102599731e0_f64 * t7659 * t8213 - 0.4336814094102599731e0_f64 * t2149 * t8217;
    t8220
}
