//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 850/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk850<F: Float>(t1445: F, t213: F, t25930: F, t25955: F, t26040: F, t26043: F, t26051: F, t26055: F, t26058: F, t27837: F, t27868: F, t27909: F, t27961: F, t27966: F, t27969: F, t27973: F, t27981: F, t561: F, t5775: F, t7279: F, t7298: F) -> (F,) {
    let t27984 = -0.65854491829355115987e0 * t27909 * t1445 + 0.8673628188205199462e0 * t27837 * t7298 + t25955 + 0.65854491829355115987e0 * t213 * t27961 * t561 + 0.54878743191129263322e-2 * t27966 + 0.9757440539382783019e-2 * t27969 - t26040 + t26043 - 0.8673628188205199462e0 * t25930 * t27973 + 0.72280234901709995518e-2 * t26051 - 0.9757440539382783019e-2 * t26055 - t26058 - 0.65854491829355115987e0 * t7279 * t5775 - 0.8673628188205199462e0 * t27868 * t27981;
    (t27984,)
}
