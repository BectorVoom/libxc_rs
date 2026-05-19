//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 935/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk935<F: Float>(t25931: F, t27972: F, t2022: F, t3999: F, t14230: F, t1445: F, t213: F, t25930: F, t25955: F, t26040: F, t26043: F, t26051: F, t26055: F, t26058: F, t27837: F, t27868: F, t27909: F, t27961: F, t27966: F, t27969: F, t561: F, t5775: F, t7279: F, t7298: F) -> (F, F) {
    let t27973 = t25931 * t27972;
    let t27980 = t3999 * t2022;
    let t27981 = t27980 * t14230;
    let t27984 = -F::cast_from(0.65854491829355115987e0_f64) * t27909 * t1445 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t7298 + t25955 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t27961 * t561 + F::cast_from(0.54878743191129263322e-2_f64) * t27966 + F::cast_from(0.9757440539382783019e-2_f64) * t27969 - t26040 + t26043 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t27973 + F::cast_from(0.72280234901709995518e-2_f64) * t26051 - F::cast_from(0.9757440539382783019e-2_f64) * t26055 - t26058 - F::cast_from(0.65854491829355115987e0_f64) * t7279 * t5775 - F::cast_from(0.8673628188205199462e0_f64) * t27868 * t27981;
    (t27980, t27984)
}
