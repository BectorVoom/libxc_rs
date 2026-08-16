//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 946/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk946(t25931: f64, t27972: f64, t2022: f64, t3999: f64, t14230: f64, t1445: f64, t213: f64, t25930: f64, t25955: f64, t26040: f64, t26043: f64, t26051: f64, t26055: f64, t26058: f64, t27837: f64, t27868: f64, t27909: f64, t27961: f64, t27966: f64, t27969: f64, t561: f64, t5775: f64, t7279: f64, t7298: f64) -> (f64, f64, f64) {
    let t27973 = t25931 * t27972;
    let t27980 = t3999 * t2022;
    let t27981 = t27980 * t14230;
    let t27984 = -0.65854491829355115987e0_f64 * t27909 * t1445 + 0.8673628188205199462e0_f64 * t27837 * t7298 + t25955 + 0.65854491829355115987e0_f64 * t213 * t27961 * t561 + 0.54878743191129263322e-2_f64 * t27966 + 0.9757440539382783019e-2_f64 * t27969 - t26040 + t26043 - 0.8673628188205199462e0_f64 * t25930 * t27973 + 0.72280234901709995518e-2_f64 * t26051 - 0.9757440539382783019e-2_f64 * t26055 - t26058 - 0.65854491829355115987e0_f64 * t7279 * t5775 - 0.8673628188205199462e0_f64 * t27868 * t27981;
    (t27973, t27981, t27984)
}
