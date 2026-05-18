//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 778/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk778<F: Float>(t1770: F, t1775: F, t1829: F, t2144: F, t2149: F, t2152: F, t460: F, t7602: F, t7632: F, t7636: F, t7643: F, t7651: F, t7659: F, t8192: F, t8198: F, t8202: F, t8205: F, t8209: F, t8213: F, t8217: F) -> F {
    let t8220 = F::new(0.65854491829355115987e0) * t1770 * t2144 - F::new(0.65854491829355115987e0) * t7602 * t1775 + F::new(0.65854491829355115987e0) * t460 * t8192 - F::new(0.65854491829355115987e0) * t7632 * t1829 - F::new(0.8673628188205199462e0) * t7636 * t8198 + F::new(0.8673628188205199462e0) * t7643 * t8202 - F::new(0.4336814094102599731e0) * t8205 * t2152 + F::new(0.8673628188205199462e0) * t7651 * t8209 - F::new(0.4336814094102599731e0) * t7659 * t8213 - F::new(0.4336814094102599731e0) * t2149 * t8217;
    t8220
}
