//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 948/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk948<F: Float>(t1841: F, t7712: F, t30046: F, t30048: F, t30051: F, t30056: F, t33876: F, t33887: F, t36833: F, t38781: F, t38787: F, t38792: F, t38796: F, t38799: F, t38801: F, t38805: F, t38810: F, t38815: F) -> (F,) {
    let t38817 = t7712 * t1841;
    let t38819 = -0.21437009059034868486e-3 * t38781 + 0.15724046144802076034e-3 * t38787 - 0.10718504529517434243e-2 * t38792 + t30046 + t30048 + t30051 + t30056 - t36833 - 0.18007087609589289529e-1 * t33876 + 0.4584375e-1 * t38796 + 0.305625e-1 * t38799 - 0.31448092289604152068e-3 * t38801 - 0.31448092289604152068e-3 * t38805 - 0.31448092289604152068e-3 * t38810 - 0.20965394859736101379e-3 * t38815 - t33887 - 0.42874018118069736972e-3 * t38817;
    (t38819,)
}
